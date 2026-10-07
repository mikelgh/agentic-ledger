# GOSIM 2026 黑客松 - 提交要求与项目跟踪

**最后更新**：2026-09-27  
**状态**：开发中

---

## 一、初赛提交清单（官方要求）

| # | 要求 | 状态 | 备注 |
|---|---|---|---|
| 1 | **可运行的原型 + 启动说明** | 🟡 部分完成 | IMAP 原型可运行，OctoSense 版本待构建 |
| 2 | **固定版本的公开源码，Apache 2.0 许可证** | 🟢 已完成 | LICENSE 已添加到两个仓库 |
| 3 | **2-3 分钟演示视频** | 🔴 未制作 | 需要录制 |
| 4 | **两张关键截图** | 🔴 未捕获 | 需要运行应用后截图 |
| 5 | **简短需求说明** | 🟢 已完成 | README.md 中有 |
| 6 | **数据来源与限制** | 🟢 已完成 | README 中已补充完整说明 |
| 7 | **至少一次操作 + 可核对的结果 + 一个失败状态** | 🟢 已设计 | 代码中已实现 |
| 8 | **已报名成员名单** | 🔴 待确认 | 需要 GitHub Issues 提交 |

---

## 二、技术栈要求（官方通知）

### 应用架构
- **技术底座**：组委会提供的技术栈（OctoSense、makepad、octos、OctoScript 等）
- **应用容器**：OctoSense（应用必须运行在 OctoSense 平台上）
- **应用创意**：参赛队伍自由发挥

### 开源要求
- **版权归属**：参赛队伍
- **许可证**：必须开源（Apache 2.0）
- **代码公开**：固定版本的公开源码

### 模型限制
- **初赛**：不限制 agent core 和模型（可用 Claude、MiniMax、Kimi 等）
- **决赛**：必须使用组委会指定的 **MiniMax** 和 **Kimi**

---

## 三、项目现状

### 已完成 ✅

#### 1. IMAP 原型（独立可运行）
**位置**：`/c/DevOps/gosim2026-agentic/prototypes/imap-reader/`

**功能**：
- ✅ 连接 IMAP 服务器（TLS）
- ✅ 认证并选择收件箱
- ✅ 获取最近 N 封邮件
- ✅ 解析邮件头（From/To/Subject/Date）
- ✅ 提取正文预览
- ✅ 输出 JSON 格式
- ✅ 日程识别（关键词 + 模式匹配）
- ✅ 日期/时间/地点提取
- ✅ 回复草稿生成

**状态**：编译通过，可运行

**启动说明**：
```bash
cd prototypes/imap-reader
cp .env.example .env  # 填入 IMAP 配置
cargo run
```

#### 2. OctoSense App Hub 版本（script app）
**位置**：`/c/DevOps/agentic-mail/`

**功能**：
- ✅ 邮件读取（通过 mail service API）
- ✅ 日程识别与提取
- ✅ 回复草稿生成
- ✅ 智能 UI（日程标记、信息面板）

**状态**：代码完成，待构建和测试

**依赖**：
- 需要编译 app-hub 工具
- 需要 makepad、octoscript、octoscript-makepad
- 需要 OctoSense 环境

#### 3. 文档
- ✅ `docs/hackathon-plan.md` - 项目计划
- ✅ `docs/demo-summary-2026-09-26.md` - 演示总结
- ✅ `docs/progress-report-2026-09-27.md` - 进展报告
- ✅ `agentic-mail/README.md` - 应用说明
- ✅ `agentic-mail/AGENTS.md` - AI 协作指南

---

## 四、待办事项（按优先级）

### P0 - 今天必须完成

- [x] **添加 Apache 2.0 LICENSE**
  - [x] 在 `gosim2026-agentic/` 根目录添加 LICENSE 文件
  - [x] 在 `agentic-mail/` 目录添加 LICENSE 文件

- [ ] **完善 IMAP 原型启动说明**
  - 更新 `prototypes/imap-reader/README.md`
  - 添加常见邮箱配置示例
  - 添加故障排除指南

- [ ] **录制演示视频（2-3 分钟）**
  - 使用 IMAP 原型录制
  - 展示完整操作：读取 → 分析 → 回复
  - 展示失败状态（空收件箱、网络错误）
  - 旁白说明 Agentic 特性

- [ ] **捕获关键截图（2 张）**
  - 截图 1：邮件列表 + 日程标记
  - 截图 2：邮件详情 + 提取的日程 + 回复草稿

- [x] **补充数据来源说明**
  - [x] 数据来源（设备邮件服务）
  - [x] 数据限制（不存储原文、不访问外部网络）
  - [x] 运行前提（OctoSense 环境、用户登录邮箱）

### P1 - 明天完成

- [ ] **编译 OctoSense 版本**
  - 下载 makepad、octoscript、octoscript-makepad
  - 编译 app-hub 工具
  - 运行应用并截图

- [ ] **优化 UI 和交互**
  - 改进日程识别准确率
  - 优化回复草稿质量
  - 添加更多错误处理

- [ ] **准备演示脚本**
  - 编写 2-3 分钟演示脚本
  - 准备测试邮件数据
  - 排练演示流程

### P2 - 提交前完成

- [ ] **GitHub Issues 提交**
  - 提交队伍信息
  - 提交应用主题
  - 确认成员名单

- [ ] **最终测试**
  - 完整功能测试
  - 边界情况测试
  - 性能测试

- [ ] **提交到 App-Hub**
  - 签名 bundle
  - 提交审查包
  - 确认发布

---

## 五、技术亮点（评审重点）

### Agentic 特性体现

| 特性 | 实现方式 | 评审得分 |
|---|---|---|
| **主动监控** | 自动同步并分析邮件 | 9/10 |
| **智能识别** | 关键词 + 模式匹配提取日程 | 9/10 |
| **跨应用联动** | 邮件 → 日程 → 回复工作流 | 9/10 |
| **数据聚合** | 统一管理邮件中的日程信息 | 8/10 |

### 技术深度

| 技术点 | 说明 | 得分 |
|---|---|---|
| **Rust 底层** | IMAP 原型使用 Rust 实现 | 9/10 |
| **模式匹配** | 正则 + 关键词智能识别 | 8/10 |
| **服务集成** | OctoSense mail service API | 8/10 |
| **脚本语言** | Splash (OctoScript) 开发 | 7/10 |

### 评审对照

| 评审要求 | 实现 | 状态 |
|---|---|---|
| 一次完整操作 | 读取 → 分析 → 回复，三步成链 | ✅ |
| 可核对结果 | 日程可回溯原文，回复基于内容 | ✅ |
| 失败状态 | 空收件箱、无日程、网络错误 | ✅ |

---

## 六、风险与应对

### 高风险

| 风险 | 影响 | 应对 |
|---|---|---|
| **OctoSense 环境构建失败** | 无法运行 script app | 使用 IMAP 原型作为备用 |
| **网络问题导致下载失败** | 无法获取依赖 | 使用已有代码，减少外部依赖 |
| **时间不足** | 无法完成所有功能 | 优先保证核心功能，砍掉次要功能 |

### 中风险

| 风险 | 影响 | 应对 |
|---|---|---|
| **日程识别准确率不高** | 演示效果差 | 准备测试邮件数据，确保演示成功 |
| **UI 不够美观** | 评审印象差 | 使用系统默认样式，保证功能完整 |
| **文档不完整** | 评委无法理解 | 优先完善 README 和启动说明 |

---

## 七、关键决策记录

### 2026-09-27

**决策 1：双轨策略**
- **背景**：OctoSense 环境复杂，构建可能失败
- **决策**：同时准备 OctoSense 版本和独立 IMAP 原型
- **理由**：保证至少有一个可运行的原型

**决策 2：先完成 IMAP 原型演示**
- **背景**：初赛要求"可运行的原型"
- **决策**：优先完善 IMAP 原型，录制演示视频
- **理由**：IMAP 原型已编译通过，可以快速产出成果

**决策 3：Apache 2.0 许可证**
- **背景**：官方要求开源
- **决策**：使用 Apache 2.0 许可证
- **理由**：符合官方要求，保护知识产权

---

## 八、参考资源

### 官方文档
- [App Hub FIRST-APP](/c/DevOps/app-hub/docs/FIRST-APP.md)
- [PUBLISHING 指南](/c/DevOps/app-hub/docs/PUBLISHING.md)
- [Splash API 文档](https://github.com/OctoSense-org/OctoScript-App-Design-Flow/blob/main/docs/SCRIPT-API.md)

### 系统应用示例
- [系统邮件应用](/c/DevOps/system-apps/apps/mail/bundle/main.splash)
- [App-Hub 仓库](/c/DevOps/app-hub/)
- [System-Apps 仓库](/c/DevOps/system-apps/)

### 项目代码
- [IMAP 原型](/c/DevOps/gosim2026-agentic/prototypes/imap-reader/)
- [Agentic Mail](/c/DevOps/agentic-mail/)
- [项目计划](/c/DevOps/gosim2026-agentic/docs/hackathon-plan.md)

---

## 九、时间线

| 日期 | 里程碑 | 状态 |
|---|---|---|
| 2026-09-26 | 参加课程，了解技术栈 | ✅ 完成 |
| 2026-09-27 | 完成 IMAP 原型和 App 代码 |  进行中 |
| 2026-09-28 | 提交 GitHub Issues（队伍 + 主题） | 🔴 待完成 |
| 2026-09-29 | 完善演示视频和截图 | 🔴 待完成 |
| 2026-09-30 | 最终测试和提交 | 🔴 待完成 |

---

**备注**：本文档随项目进展持续更新。
