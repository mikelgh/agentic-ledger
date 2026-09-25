# GOSIM 2026 Agentic App 黑客松 — 参赛规划

> 队伍：睿欣工场（1 人 + AI）
> 项目：agentic-ledger — Agentic 消费管理
> 底座：Octos (OctoCode) — 基于 Rust 的可嵌入 Agent 内核
> 官网：https://create.gosim.org/agenticapp26/
> 启动会录制：https://meeting.tencent.com/crm/N1XqyaeE99

---

## 1. 初赛参赛流程

> 指定仓库：`gosimfoundation/hackathon-agenticapp26`（步骤 1 和 4 都指向同一个仓库）

| 步骤 | 内容 | 状态 |
|---|---|---|
| 1 | 在指定 GitHub 仓库通过 issues 确认队伍和参赛主题 | [ ] 待 9/26 课程后确认提交方式 |
| 2 | 参赛主题通过后，注册 minimax 指定账号（周末课后发放） | [ ] 等待发放 |
| 3 | 按天/按队伍领取 token，参加初赛 | [ ] 未开始 |
| 4 | 初赛作品提交到指定 GitHub 仓库 | [ ] 待确认：PR 形式 or Issue 链接形式？ |

**待 9/26 课程确认：**
- [ ] 作品提交方式：Fork + PR？还是在 Issue 里贴自建仓库链接？
- [ ] 是否需要放在主办方 monorepo 的子目录里？
- [ ] 提交格式/目录结构要求？

**开发仓库（非提交目标，仅日常开发用）：**
- GitHub: https://github.com/mikelgh/agentic-ledger （主开发）
- Gitee: https://gitee.com/mikelgh/agentic-ledger （镜像备份）

---

## 2. 课程安排（9/26 周六）

| 时间 | 主题 | 与本项目的关系 |
|---|---|---|
| 14:00–16:00 | OctoSense × Makepad：让意图成为应用 | UI 层 — 当前 README 标记"待课程+官方环境包" |
| 16:30–18:30 | octoscode × Octoscript：和 Agent 一起做应用 | Agent 编排层 — 当前 AutoSense OS / OctoScript 接入方式待确认 |

**课后待办：**
- [ ] 获取 Makepad 集成方式（OUP 协议？官方环境包？）
- [ ] 获取 OctoScript 接入方式
- [ ] 确认 minimax 账号和 token 发放时间

---

## 3. 项目当前状态

### 已完成
- [x] Rust 工具链就绪（1.98.1 stable-msvc）
- [x] 核心业务逻辑完成：`ingest_expenses` + `analyze_spending`
- [x] CLI 二进制：JSON in → JSON out，非零退出 = 失败
- [x] workspace-policy.toml 声明式契约
- [x] manifest.json 工具清单
- [x] SKILL.md Agent 技能文档
- [x] 8 个契约测试全绿（含 4 段式 smoke test + 失败路径）
- [x] 示例数据和报告已生成
- [x] `cargo test` 全绿（9/25 验证通过）
- [x] GitHub + Gitee 双仓库推送就绪

### 进行中 / 待完成
- [ ] Makepad UI 层（待 9/26 课程 + 官方环境包）
- [ ] AutoSense OS / OctoScript 接入方式确认
- [ ] 确认作品提交方式（PR / Issue 链接 / 其他）
- [ ] GitHub issues 提交队伍和主题
- [ ] minimax 账号注册

---

## 4. 评审要求 ↔ 项目落点

初赛评审要三样看得见的东西：

| 要求 | 本项目落点 | 验证方式 |
|---|---|---|
| 一次完整操作 | `ingest_expenses` → `analyze_spending`，两步成链 | CLI 手动走一遍 |
| 可核对的结果 | 金额以整数分存储，报告每行可回溯 `data/transactions.json` | 人工复算 |
| 失败或空状态 | 空输入 / 未入库 / 空数据集三条路径 → harness `failed` | 测试覆盖 |

---

## 5. 近期行动项

| 优先级 | 行动 | 截止 | 备注 |
|---|---|---|---|
| P0 | 参加 9/26 课程，记录 Makepad 和 OctoScript 接入要点 | 9/26 | 核心依赖 |
| P0 | 课程上确认作品提交方式（PR / Issue 链接 / 目录结构） | 9/26 | 决定后续流程 |
| P1 | GitHub issues 提交队伍+主题 | 课程后 | 步骤 1 |
| P1 | 注册 minimax 账号 | 收到链接后 | 步骤 2 |
| P2 | Makepad UI 层接入 | 课程后 | 需官方环境包 |
| P2 | OctoScript 接入 | 课程后 | 需官方环境包 |
| P3 | 准备演示脚本（完整操作 + 失败状态） | 提交前 | 评审要求 |

---

## 6. 架构概览

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
  通过 → lifecycle_state=Ready
  失败 → lifecycle_state=Failed → notify_user
```

关键设计约束：金额以整数分（cents）存储求和，报告可逐条核对。
