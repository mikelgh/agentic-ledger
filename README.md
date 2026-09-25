# agentic-ledger · GOSIM 2026 Agentic App 黑客松

> 一句话：给一段银行流水和一个问题，Agent 入库、分类、产出一份**每个数字都能
> 逐条回溯到原始行**的支出报告。

参赛队：睿欣工场（1 人 + AI）
赛题方向：Agentic 购物与消费管理
构建于 [Octos](https://github.com/octos-org/octos-arc) —— 比赛宣讲里的 **OctoCode**，
"基于 Rust 的可嵌入 Agent 内核"。

---

## 为什么是这个选题

初赛评审要的是三样看得见的东西：**一次完整操作 · 一个可核对的结果 · 一个失败
或空状态**。消费管理天然满足：

| 要求 | 本项目的落点 |
|---|---|
| 一次完整操作 | `ingest_expenses` → `analyze_spending`，两步成链 |
| 可核对的结果 | 金额以**整数分**存储求和，报告每行可由 `data/transactions.json` 复算 |
| 失败或空状态 | 空输入 / 未入库 / 空数据集三条路径都通向 harness 的 `failed`，而非交付一份看起来正常的空报告 |

"可核对"是设计约束而非副产品：用浮点存钱、或者让模型直接生成一段看起来合理的
散文，都做不到复核。

## 架构：不自己造 harness

验证/修复循环**不是自研的**，是 Octos harness 的声明式契约：

```
用户意图（自然语言）
      ↓
  Octos Agent 规划
      ↓
  ingest_expenses  ──→  data/transactions.json   (role: dataset)
      ↓
  analyze_spending ──→  reports/spending-*.md    (role: primary)
      ↓
  harness on_verify: file_exists + file_size_min:256
      ↓
  通过 → lifecycle_state=Ready，运行时负责交付
  失败 → lifecycle_state=Failed，触发 on_failure 的 notify_user
```

关键：`workspace-policy.toml` 里声明"什么必须为真才算完成"，运行时负责强制。
任务在必需验证器失败时**无法**报告 ready——这不是提示词约定，是契约。

## 文件地图

```
agentic-ledger/
├── Cargo.toml               # path 依赖 ../octos-arc-ref/crates/*（dev 用）
├── manifest.json            # 插件清单：两个 tool 的 input_schema
├── SKILL.md                 # 技能文档（Agent 读的）
├── workspace-policy.toml    # ★ harness 契约：artifacts / validation / spawn_tasks
├── src/
│   ├── lib.rs               # 工具逻辑：解析、分类、聚合、渲染
│   └── main.rs              # CLI：JSON in → JSON out，非零退出 = 失败
├── tests/harness_smoke.rs   # 契约要求的 4 段式 smoke test + 失败路径
└── examples/                # 演示输入
```

## 跑起来

```bash
# 契约测试：无需 API key、无网络、无外部依赖
cargo test

# 手动走一遍完整链路
cargo run -- ingest_expenses  < examples/ingest.json
cargo run -- analyze_spending < examples/analyze.json
cat reports/spending-*.md
```

## 当前状态

- [x] Rust 工具链（1.98.1 stable-msvc）
- [x] 核心业务逻辑完成：`ingest_expenses` + `analyze_spending`
- [x] CLI 二进制：JSON in → JSON out，非零退出 = 失败
- [x] `cargo test` 全绿（8 个契约测试，无需 API key / 网络）
- [x] workspace-policy.toml 声明式契约
- [x] manifest.json 工具清单 + SKILL.md Agent 技能文档
- [x] 示例数据与报告已生成
