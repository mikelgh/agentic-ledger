//! Agentic 消费管理 — harnessed custom app logic.
//!
//! Two tools, mirroring the octos harness contract
//! (`octos-arc-ref/docs/OCTOS_HARNESS_DEVELOPER_GUIDE.md`):
//!
//! - `ingest_expenses`  : raw transaction lines -> structured `data/transactions.json`
//! - `analyze_spending` : structured records -> `reports/spending-<slug>.md` (primary artifact)
//!
//! Amounts are carried as integer cents so the report's totals are exactly
//! reproducible from the input, which is what makes the delivered artifact a
//! checkable result rather than prose.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use eyre::{Result, WrapErr};
use serde::{Deserialize, Serialize};

/// One parsed transaction. `amount_cents` is signed: expenses are positive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transaction {
    pub date: String,
    pub description: String,
    pub amount_cents: i64,
    pub category: String,
}

#[derive(Debug, Deserialize)]
pub struct IngestInput {
    /// Raw ledger text: one transaction per line.
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IngestOutput {
    pub data_path: PathBuf,
    pub record_count: usize,
    pub skipped_lines: usize,
}

#[derive(Debug, Deserialize)]
pub struct AnalyzeInput {
    /// Free-text label for the report; also derives the filename.
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalyzeOutput {
    pub artifact_path: PathBuf,
    pub total_cents: i64,
    pub record_count: usize,
}

const DATA_REL: &str = "data/transactions.json";

/// Parse raw ledger text into `data/transactions.json` under `workspace_root`.
///
/// Fails when no line yields a usable record, so the harness validator drives
/// the task to `failed` instead of delivering an empty report.
pub fn ingest_expenses(workspace_root: &Path, input: &IngestInput) -> Result<IngestOutput> {
    let mut records = Vec::new();
    let mut skipped = 0usize;

    for line in input.text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        match parse_line(line) {
            Some(tx) => records.push(tx),
            None => skipped += 1,
        }
    }

    if records.is_empty() {
        eyre::bail!(
            "no transactions parsed from {} input line(s); nothing to ingest",
            input.text.lines().count()
        );
    }

    records.sort_by(|a, b| a.date.cmp(&b.date).then_with(|| a.description.cmp(&b.description)));

    let data_path = workspace_root.join(DATA_REL);
    if let Some(parent) = data_path.parent() {
        std::fs::create_dir_all(parent)
            .wrap_err_with(|| format!("create data dir failed: {}", parent.display()))?;
    }
    let json =
        serde_json::to_string_pretty(&records).wrap_err("serialize transactions failed")?;
    std::fs::write(&data_path, json)
        .wrap_err_with(|| format!("write data failed: {}", data_path.display()))?;

    Ok(IngestOutput {
        data_path: PathBuf::from(DATA_REL),
        record_count: records.len(),
        skipped_lines: skipped,
    })
}

/// Aggregate `data/transactions.json` into a markdown report under `reports/`.
pub fn analyze_spending(workspace_root: &Path, input: &AnalyzeInput) -> Result<AnalyzeOutput> {
    let data_path = workspace_root.join(DATA_REL);
    let raw = std::fs::read_to_string(&data_path).wrap_err_with(|| {
        format!(
            "read {DATA_REL} failed; run ingest_expenses first ({})",
            data_path.display()
        )
    })?;
    let records: Vec<Transaction> =
        serde_json::from_str(&raw).wrap_err("data/transactions.json is not valid")?;

    if records.is_empty() {
        eyre::bail!("{DATA_REL} holds no records; refusing to emit an empty report");
    }

    let total_cents: i64 = records.iter().map(|t| t.amount_cents).sum();
    let mut by_category: BTreeMap<String, (i64, usize)> = BTreeMap::new();
    for tx in &records {
        let entry = by_category
            .entry(tx.category.clone())
            .or_insert((0, 0));
        entry.0 += tx.amount_cents;
        entry.1 += 1;
    }

    let mut ranked: Vec<(&String, (i64, usize))> =
        by_category.iter().map(|(k, v)| (k, *v)).collect();
    ranked.sort_by(|a, b| b.1.0.cmp(&a.1.0).then_with(|| a.0.cmp(b.0)));

    let mut top: Vec<&Transaction> = records.iter().collect();
    top.sort_by(|a, b| b.amount_cents.cmp(&a.amount_cents));

    let first_date = records
        .iter()
        .map(|t| t.date.as_str())
        .min()
        .unwrap_or("unknown");
    let last_date = records
        .iter()
        .map(|t| t.date.as_str())
        .max()
        .unwrap_or("unknown");

    let rendered = render_report(
        &input.label,
        first_date,
        last_date,
        records.len(),
        total_cents,
        &ranked,
        &top,
    );

    let slug = slugify(&input.label);
    let relative = Path::new("reports").join(format!("spending-{slug}.md"));
    let full = workspace_root.join(&relative);
    if let Some(parent) = full.parent() {
        std::fs::create_dir_all(parent)
            .wrap_err_with(|| format!("create reports dir failed: {}", parent.display()))?;
    }
    std::fs::write(&full, rendered.as_bytes())
        .wrap_err_with(|| format!("write report failed: {}", full.display()))?;

    Ok(AnalyzeOutput {
        artifact_path: relative,
        total_cents,
        record_count: records.len(),
    })
}

fn render_report(
    label: &str,
    first_date: &str,
    last_date: &str,
    record_count: usize,
    total_cents: i64,
    ranked: &[(&String, (i64, usize))],
    top: &[&Transaction],
) -> String {
    let mut out = String::new();
    out.push_str(&format!("# 消费分析报告：{label}\n\n"));
    out.push_str(&format!("- 统计区间：{first_date} ~ {last_date}\n"));
    out.push_str(&format!("- 交易笔数：{record_count}\n"));
    out.push_str(&format!("- 总支出：{}\n\n", format_cents(total_cents)));

    out.push_str("## 分类汇总\n\n");
    out.push_str("| 分类 | 金额 | 占比 | 笔数 |\n|---|---:|---:|---:|\n");
    for (category, (cents, count)) in ranked {
        out.push_str(&format!(
            "| {category} | {} | {} | {count} |\n",
            format_cents(*cents),
            share_of(*cents, total_cents)
        ));
    }

    out.push_str("\n## 金额最高的 5 笔\n\n");
    for tx in top.iter().take(5) {
        out.push_str(&format!(
            "- {} {} · {} · {}\n",
            tx.date,
            tx.description,
            format_cents(tx.amount_cents),
            tx.category
        ));
    }

    out.push_str(
        "\n---\n每行的金额、占比与笔数均可由 data/transactions.json 逐条核对；\
         分类为规则匹配，规则见 agentic_ledger::categorize。\n",
    );
    out
}

/// Map a description to a spending category by keyword.
///
/// Order matters: the first matching rule wins, so narrower rules come first.
pub fn categorize(description: &str) -> &'static str {
    const RULES: &[(&[&str], &str)] = &[
        (&["地铁", "公交", "打车", "滴滴", "出租", "网约", "高铁", "机票", "加油", "uber", "taxi", "metro"], "交通"),
        (&["房租", "物业", "水费", "电费", "燃气", "宽带", "rent", "utility"], "居住"),
        (&["外卖", "美团", "饿了么", "餐", "饭", "咖啡", "星巴克", "luckin", "超市", "买菜", "food", "grocery"], "餐饮"),
        (&["药", "医院", "门诊", "体检", "诊所", "卫生", "疫苗", "pharmacy", "clinic"], "医疗"),
        (&["电影", "游戏", "会员", "视频", "演出", "健身", "netflix", "steam"], "娱乐"),
        (&["衣服", "数码", "手机", "电脑", "日用", "淘宝", "京东", "拼多多", "shop", "store"], "购物"),
        (&["学费", "书", "课程", "培训", "book"], "教育"),
    ];
    let lowered = description.to_lowercase();
    for (keywords, category) in RULES {
        if keywords.iter().any(|k| lowered.contains(&k.to_lowercase())) {
            return category;
        }
    }
    "其他"
}

/// Parse `date description amount`. The amount is the last numeric token, the
/// date the first `YYYY-MM-DD` token, and the description whatever is between.
fn parse_line(line: &str) -> Option<Transaction> {
    let tokens: Vec<&str> = if line.contains(',') {
        line.split(',').map(str::trim).collect()
    } else {
        line.split_whitespace().collect()
    };
    if tokens.len() < 3 {
        return None;
    }

    let amount_cents = parse_amount_cents(*tokens.last()?)?;
    let date = tokens[0];
    if !looks_like_date(date) {
        return None;
    }
    let description = tokens[1..tokens.len() - 1].join(" ");
    if description.is_empty() {
        return None;
    }

    Some(Transaction {
        date: date.to_string(),
        category: categorize(&description).to_string(),
        description,
        amount_cents,
    })
}

fn parse_amount_cents(raw: &str) -> Option<i64> {
    let cleaned: String = raw
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == '.' || *c == '-')
        .collect();
    if cleaned.is_empty() || cleaned == "-" || cleaned == "." {
        return None;
    }
    let value: f64 = cleaned.parse().ok()?;
    Some((value * 100.0).round() as i64)
}

fn looks_like_date(token: &str) -> bool {
    let bytes = token.as_bytes();
    bytes.len() == 10
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes
            .iter()
            .enumerate()
            .all(|(i, b)| if i == 4 || i == 7 { *b == b'-' } else { b.is_ascii_digit() })
}

fn format_cents(cents: i64) -> String {
    let sign = if cents < 0 { "-" } else { "" };
    let abs = cents.unsigned_abs();
    format!("{sign}¥{}.{:02}", abs / 100, abs % 100)
}

fn share_of(part: i64, whole: i64) -> String {
    if whole == 0 {
        return "—".to_string();
    }
    let pct = (part as f64 / whole as f64) * 100.0;
    format!("{pct:.1}%")
}

pub fn slugify(label: &str) -> String {
    let mut out = String::with_capacity(label.len());
    let mut last_dash = false;
    for c in label.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c.to_ascii_lowercase());
            last_dash = false;
        } else if c.is_alphanumeric() {
            // Keep non-ASCII labels usable as filenames without escaping.
            out.push(c);
            last_dash = false;
        } else if !last_dash {
            out.push('-');
            last_dash = true;
        }
    }
    let trimmed = out.trim_matches('-').to_string();
    if trimmed.is_empty() { "ledger".to_string() } else { trimmed }
}
