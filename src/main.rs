//! CLI binary for `agentic-ledger`.
//!
//! Plugin binary protocol: `./agentic-ledger <tool_name>` with JSON on stdin
//! and exactly one JSON object on stdout. Diagnostics go to stderr.
//! A non-zero exit tells the harness the task failed, which routes it to the
//! policy's `on_failure` actions instead of delivering an artifact.

use std::io::Read;
use std::path::PathBuf;

use agentic_ledger::{AnalyzeInput, IngestInput, analyze_spending, ingest_expenses};
use serde_json::json;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let tool_name = args.get(1).map(String::as_str).unwrap_or("");

    let mut raw = String::new();
    if let Err(err) = std::io::stdin().read_to_string(&mut raw) {
        reply_failure(&format!("read stdin failed: {err}"));
        return;
    }

    match tool_name {
        "ingest_expenses" => handle_ingest(&raw),
        "analyze_spending" => handle_analyze(&raw),
        other => reply_failure(&format!(
            "unknown tool '{other}', expected: ingest_expenses | analyze_spending"
        )),
    }
}

fn workspace_root() -> PathBuf {
    std::env::var_os("OCTOS_WORKSPACE_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
}

fn handle_ingest(raw: &str) {
    let input: IngestInput = match serde_json::from_str(raw) {
        Ok(value) => value,
        Err(err) => return reply_failure(&format!("invalid input JSON: {err}")),
    };
    match ingest_expenses(&workspace_root(), &input) {
        Ok(out) => print_success(
            &format!(
                "Ingested {} transaction(s) to {} ({} unparseable line(s) skipped)",
                out.record_count,
                out.data_path.display(),
                out.skipped_lines
            ),
            &[out.data_path.to_string_lossy().into_owned()],
        ),
        Err(err) => reply_failure(&format!("ingest_expenses failed: {err:?}")),
    }
}

fn handle_analyze(raw: &str) {
    let input: AnalyzeInput = match serde_json::from_str(raw) {
        Ok(value) => value,
        Err(err) => return reply_failure(&format!("invalid input JSON: {err}")),
    };
    match analyze_spending(&workspace_root(), &input) {
        Ok(out) => print_success(
            &format!(
                "Wrote {} covering {} transaction(s), total {}",
                out.artifact_path.display(),
                out.record_count,
                format_cents(out.total_cents)
            ),
            &[out.artifact_path.to_string_lossy().into_owned()],
        ),
        Err(err) => reply_failure(&format!("analyze_spending failed: {err:?}")),
    }
}

fn format_cents(cents: i64) -> String {
    let sign = if cents < 0 { "-" } else { "" };
    let abs = cents.unsigned_abs();
    format!("{sign}¥{}.{:02}", abs / 100, abs % 100)
}

fn print_success(message: &str, files: &[String]) {
    println!(
        "{}",
        json!({ "success": true, "output": message, "files_to_send": files })
    );
}

fn reply_failure(message: &str) {
    eprintln!("{message}");
    println!("{}", json!({ "success": false, "output": message }));
    std::process::exit(1);
}
