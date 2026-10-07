# GOSIM 2026 Agentic App 黑客松 — 参赛规划

> 队伍：睿欣达工场（1 人 + AI）
> 项目：**agentic-mail** — OctoSense 邮件应用（原 agentic-ledger，已切换）
> 场景：**OctoSense 邮件** — 12 个 OctoSense 场景之一
> 底座：Octos (OctoCode) — 基于 Rust 的可嵌入 Agent 内核
> 定位：基于 OctoSense 平台构建的 Agentic 邮件应用，非独立 IMAP 工具
> 官网：https://create.gosim.org/agenticapp26/
> 启动会录制：https://meeting.tencent.com/crm/N1XqyaeE99
> 研讨课 #1 录制：https://meeting.tencent.com/crm/2kAd7Z94bd
> 研讨课 #1 纪要：[2026-09-26](course-notes-2026-09-26.md)
> 选题决策：[从消费管理切换到 OctoSense 邮件](decisions/001-topic-switch-to-email.md)

---

## 1. 初赛参赛流程

> 队伍信息登记：`gosimfoundation/hackathon-agenticapp26` Issues
> 作品提交：`OctoSense-org/OctoSense-App-Hub` Issues → 维护者 hub publish

| 步骤 | 内容 | 状态 |
|---|---|---|
| 1 | 在 gosimfoundation 仓库 Issues 登记队伍和参赛主题 | [x] 已完成 |
| 2 | 代码推 GitHub 打 tag → 向 OctoSense-App-Hub 提 Issue | [x] Issue #33 已提交 |
| 3 | 维护者执行 hub publish 完成发布 | [ ] 等待维护者处理 |
| 4 | 评委导入 Rinks 小程序人工运行评审 | [ ] 10/4-10/6 晋级窗口 |

**关键时间节点：**
- **10/4–10/6**：初赛晋级窗口
- **10/9**：冻结
- **10/12**：决赛

**作品提交要点（课程 #2 确认，当前权威）：**
- 代码推到 GitHub 打 tag → 向 OctoSense-App-Hub 提 issue → 维护者 hub publish
- 评审由评委导入 Rinks 小程序人工运行，非 AI 自动评分
- 初赛代码只要求 OctoScript 应用（bundle 形态），不要求独立 Rust 原型
- 队伍信息须与群内昵称前缀一致（用于管控 token 成本）

**参赛仓库：**
- GitHub: https://github.com/mikelgh/agentic-mail （唯一提交仓库，tag v0.2.0）
- 旧仓库 agentic-ledger 已废弃

---

## 2. 研讨课 #1 关键收获（9/26）

### 2.1 技术架构全貌

| 组件 | 定位 | 备注 |
|---|---|---|
| **OctoSense (auto science)** | OS 窗口合成器，跨平台 | 比赛技术底座，支持鸿蒙/安卓 |
| **OctoCode (octoscode)** | 可嵌入 Agent 内核，纯 Rust | C/S 架构，支持 TUI 和 Web UI |
| **OctoScript (autoscript)** | AI 原生脚本语言，与 Rust 交互 | 专为大模型生成设计，支持流式响应 |
| **App Card** | 轻量容器/卡片，可通过对话生成 | 数据展示窗口，支持动态生成+流式渲染 |
| **独立进程应用** | 独立运行的 native app | 可通过接口共享数据供 App Card 读取 |
| **Makepad** | UI 渲染引擎 | 支持 GPU 渲染、地图等 UI 效果 |

### 2.2 双环 Loop 机制（核心开发模式）

```
外环（指挥官）: Claude / Codex 等强模型
  ├── 全局规划 & 审查
  ├── 任务分发 & 进度监控
  └── 不直接操作代码

内环（执行者）: MiniMax / GLM Flash 等便宜模型
  ├── 具体编码任务
  ├── 可多进程并行（6+ 进程）
  └── 无头模式默认运行，加参数可看可视化
```

- 外环额度耗尽不影响内环继续运行（解耦）
- 建议：强模型做指挥（决策），便宜模型干重活（编码）
- 内环禁止自动提交代码，外部资料由外环供给（内外隔离）

### 2.3 赛道与场景

- **赛道一**：基于 OctoSense 的应用，12 个场景自选（天气、邮件、日历、消费管理等）
- **赛道二**："remix" — 即时应用
- 本队目标：同时角逐 **"最佳 Agentic"** + **"最佳技术突破"**
- 12 个场景均需构建 **agentic 进程类应用**
- 核心要求：将场景接入 OctoSense，实现应用 agentic 化

### 2.4 评审标准

| 维度 | 说明 |
|---|---|
| **创意** | 应用场景的创新性 |
| **技术** | 底层技术深度，刷底层代码是加分项 |
| **稳定性** | Rust 编译保障，AI 生成代码由编译器把关 |

- 初赛只需 **MVP**，切勿过度开发
- 技术突破可冲击一等奖，无突破则从创意奖选拔
- 纯静态界面演示行不通，必须跑通动态全链路

### 2.5 Agentic 应用核心理念

- **从"人找服务"到"服务找人"**：Agent 主动监控日历/邮件等变化，自动推送
- **跨应用联动**：通过调用总线实现，各应用注册工具到共享池
- **数据聚合**：Agent 统一管理个人数据，打破应用孤岛
- **意图到应用**：AI 将带歧义的自然语言意图解构为结构化需求，生成应用

---

## 3. 项目当前状态

### 已完成
- [x] Rust 工具链就绪（1.98.1 stable-msvc）
- [x] Octos 契约测试框架理解与验证
- [x] **选题决策完成**：切换到 Email 场景
- [x] **GitHub Issues 提交队伍+主题**（9/27 完成）
- [x] **IMAP 邮件读取原型完成**（9/27），含日程识别/事件提取/回复草稿
- [x] **OctoSense App Hub 工作区搭建**（5 sibling 目录 + hub.exe/card-host.exe）
- [x] **agentic-mail bundle 通过 hub check**（capabilities: mail + storage, 16MB）
- [x] **代码推送至 GitHub**（https://github.com/mikelgh/agentic-mail，tag v0.2.0）
- [x] **Issue #33 提交至 OctoSense-App-Hub**
- [x] **card-host 本地实测通过**（Windows D3D11，演示模式 + 真实邮箱）
- [x] **UI 布局优化 v0.2.0**（发件人/时间不重叠、短格式时间、智能预览）
- [x] **真实截图替换占位图**（tools/octo shot 截取）

### 待完成
- [ ] **【P0】issue 评论区提交队伍信息**（睿欣工场 + 仓库地址）
- [ ] **【P0】录制演示视频**（2-3 分钟，展示完整操作链路）
- [ ] 演示邮箱真实邮件收发验证（agentic_mail_demo@163.com）
- [ ] 决赛准备：MiniMax + Kimi 运行时接入

---

## 4. 评审要求 ↔ 项目落点

初赛评审要三样看得见的东西：

| 要求 | 本项目落点（邮件场景） | 验证方式 |
|---|---|---|
| 一次完整操作 | `fetch_emails` → `summarize_email` → `extract_events`，三步成链 | CLI 手动走一遍 |
| 可核对的结果 | 摘要可回溯原文，日程可回溯邮件元数据 | 人工比对 |
| 失败或空状态 | 空收件箱 / 无权限 / 解析失败三条路径 → harness `failed` | 测试覆盖 |

**研讨课 #1 补充的评审要点：**
- 必须跑通动态全链路（非静态演示）
- 需体现 Agentic 特性（主动监控、跨应用联动）
- 技术深度是加分项（底层 Rust / 编译器保障稳定性）

**邮件场景的 Agentic 特性落点：**
- **主动监控**：Agent 定期检查收件箱，推送紧急邮件摘要
- **跨应用联动**：邮件日程 → 日历、邮件消费 → 记账、邮件地址 → 导航
- **数据聚合**：统一管理邮件中的日程、待办、消费信息

---

## 5. 近期行动项

| 优先级 | 行动 | 截止 | 备注 |
|---|---|---|---|
| **P0** | **issue 评论区提交队伍信息** | 尽快 | ✅ 已完成（睿欣达工场） |
| **P0** | **录制演示视频** | 10/4 前 | 2-3 分钟，收件箱→阅读→行动卡片→回复 |
| **P0** | **真实截图替换** | 10/4 前 | card-host 运行后 octo shot |
| P1 | 演示邮箱真实收发验证 | 10/4 前 | agentic_mail_demo@163.com |
| P2 | 决赛 MiniMax+Kimi 接入 | 10/9 前 | 运行时模型 |
| P3 | UI 进一步打磨 | 迭代 | 阅读器/回复页优化 |

---

## 6. 架构概览

```
用户意图（自然语言，带歧义）
      ↓
  Octos Agent 规划（意图解构 → 结构化需求）
      ↓
  ┌─────────────────────────────────────┐
  │  双环 Loop                           │
  │  外环: Claude/Codex（规划+审查）      │
  │  内环: MiniMax/GLM Flash（编码执行）  │
  └─────────────────────────────────────┘
      ↓
  fetch_emails     ──→  data/emails.json           (role: dataset)
      ↓
  summarize_email  ──→  reports/summary-*.md       (role: primary)
  extract_events   ──→  data/events.json           (role: dataset)
      ↓
  harness on_verify: file_exists + file_size_min:256
      ↓
  通过 → lifecycle_state=Ready
  失败 → lifecycle_state=Failed → notify_user
      ↓
  【Agentic 特性】
  Agent 主动监控收件箱 → 推送紧急邮件摘要
  跨应用联动：邮件日程 → 日历、邮件消费 → 记账
      ↓
  App Card / 独立进程应用 → OctoSense UI 渲染
```

**应用形态选择：**
- **App Card**：轻量容器，对话生成，适合数据展示（邮件摘要、日程列表等）
- **独立进程应用**：完整功能，通过接口共享数据供 App Card 读取
- 建议两者结合：进程应用做邮件处理，App Card 做展示

**关键设计约束：**
- 邮件数据本地存储，不上传云端（隐私）
- 摘要可回溯原文，日程可回溯邮件元数据（可核对）
- Rust 编译器保障 AI 生成代码的类型安全

---

## 7. 工具链速查

| 工具 | 用途 | 安装/获取 |
|---|---|---|
| OctoCode (octoscode) | Agent 内核，TUI/Web UI | 官方仓库 |
| OctoLoop | 双环 loop 管理 | octoscode 内置 |
| MiniMax | 内环主力模型 | 官方发放 token |
| Claude / Codex | 外环指挥官 | 自备 |
| Makepad | UI 渲染引擎 | 随 OctoSense |
| herd | 多终端面板管理 | 从 agentic 安装（非 PR 生态） |
| AutoScript | AI 原生脚本语言 | 随 OctoSense |
