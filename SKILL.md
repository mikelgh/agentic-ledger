---
name: agentic-ledger
description: Agentic 消费管理。把原始流水变成可逐条核对的分类支出报告。Use when the user wants to understand where their money went.
version: 0.1.0
author: 睿欣工场
always: false
---

# Agentic Ledger

意图驱动的记账应用。用户给一段流水和一个问题，Agent 入库、分类、产出一份
每个数字都能回溯到原始行的报告。

设计约束：报告的每个金额都以整数分（cents）存储和求和，因此汇总值可被人工
逐条核对——"生成了一段看起来合理的话"不算交付。

## Tools

### ingest_expenses

把原始流水解析成结构化记录，写入 `data/transactions.json`。

```json
{"text": "2026-09-01 美团外卖 45.90\n2026-09-02 地铁 6.00"}
```

**参数：**
- `text` (必填)：每行一笔，格式 `YYYY-MM-DD 描述 金额`。空行与 `#` 开头的行忽略。

**行为：**
- 金额取该行最后一个数字 token；日期取第一个 `YYYY-MM-DD` token；中间是描述
- 无法解析的行计数后跳过，不中断
- **零条可解析记录时直接失败**（退出码非零），不落盘、不产出空报告

### analyze_spending

聚合成 `reports/spending-<label>.md`。后台任务，受 harness 监督。

```json
{"label": "9月第一周"}
```

**参数：**
- `label` (必填)：报告标题，同时决定文件名

**产物：**
- 统计区间、交易笔数、总支出
- 分类汇总表（金额 / 占比 / 笔数）
- 金额最高的 5 笔
- 结尾声明：所有数字可由 `data/transactions.json` 逐条核对

## 分类规则

`categorize()` 按关键词首次命中归类，规则顺序有意义（窄规则在前）：
交通 / 居住 / 餐饮 / 医疗 / 娱乐 / 购物 / 教育 / 其他。

未命中任何关键词的落 `其他`，不静默丢弃。

## 失败与空状态

三条路径都通向 harness 的 `failed`，而不是交付一份看起来正常的空报告：

| 情况 | 拦截点 |
|---|---|
| 输入无有效行 | `ingest_expenses` 返回 Err，不落盘 |
| 未先入库就分析 | `analyze_spending` 读不到 `data/transactions.json` |
| 数据集为空 | `analyze_spending` 显式拒绝产出 |

策略里的 `on_verify = ["file_exists:$primary", "file_size_min:$primary:256"]`
是兜底：任一命中，运行时拒绝标记 ready，并触发 `on_failure` 的 `notify_user`。

## 本地跑

```bash
cargo test                        # smoke test，无需 API key、无网络
cargo run -- ingest_expenses < examples/ingest.json
cargo run -- analyze_spending < examples/analyze.json
```
