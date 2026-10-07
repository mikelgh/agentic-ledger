# GOSIM 2026 黑客松参赛作品竞争力分析

**分析时间**: 2026-10-05  
**分析对象**: OctoSense-App-Hub 已提交作品 vs agentic-mail

---

## 一、已提交作品总览（截至 10/5）

| # | 作品 ID | 版本 | 提交日期 | 赛道 | 仓库 |
|---|---------|------|----------|------|------|
| 1 | **vibemail** | 0.7.25 | 10/5 | 邮件 | yzbtdiy/VibeMail |
| 2 | oncue-screening-room | 0.4.8 | 10/4 | 观影 | - |
| 3 | com.oma.octosense.calendar | 0.4.1 | 10/4 | 日历 | mark-liuzh/OctoSense-Calendar-OMA |
| 4 | shiyi | 0.3.2 | 10/4 | 未知 | - |
| 5 | **no-reminder-agent** | 0.5.1 | 10/4 | 邮件 | nineanswerer/octo |
| 6 | octostudio | 0.3.3 | 10/4 | 工具 | - |
| 7 | traceshop.shopping | 0.4.0 | 10/3 | 购物 | prettygirlisnotme/TraceShop |
| 8 | os.memory | 0.1.0 | 10/3 | 系统 | Thneoly/os-memory |
| 9 | memory-digest | 0.1.0 | 10/3 | 记忆 | Thneoly/memory-digest |
| 10 | memory-notes | 0.1.0 | 10/3 | 记忆 | Thneoly/memory-notes |
| 11 | vibemail | 0.6.10 | 10/1 | 邮件 | yzbtdiy/VibeMail |
| 12 | **agentic-mail** | 0.2.0 | 9/30 | 邮件 | mikelgh/agentic-mail |
| 13 | liyu-mini | 1.0.0 | 9/28 | 购物 | - |

**总计**: 13 个提交，涉及约 10 个独立作品

---

## 二、邮件赛道竞品深度对比

### 2.1 VibeMail (yzbtdiy) ⭐ 最强竞品

| 维度 | VibeMail 0.7.25 | agentic-mail 0.4.1 |
|------|-----------------|-------------------|
| **版本成熟度** | 0.7.25（迭代 7 次） | 0.4.1（迭代 4 次） |
| **UI 设计** | 五屏设计（收件箱/星轨/详情/写信/设置），暖纸×炭笔双色主题，vibe 滤镜芯片 | 三页设计（收件箱/详情/写信），简洁风格 |
| **AI 功能** | 三行摘要、语气重写（5 档滑杆）、心情播报 | 规则引擎分类（日程/订单/变更）、LLM 分析（待验证） |
| **特色功能** | 星轨 Orbit（气泡大小=重要性，颜色=vibe，sin/cos 漂移） | 行动卡片（日程→日历、订单→追踪、变更→旧/新对比） |
| **能力声明** | mail + model + storage | mail + storage（model 待验证） |
| **真实验证** | 真实邮箱验证通过，AI 摘要在真实邮件上测试 | 真实邮箱同步通过，LLM 路径未闭环 |
| **工程化** | 完整 evidence 目录、SUBMISSION.md、regression 测试 | 基础文档、无回归测试 |

**VibeMail 优势**:
- UI 设计更精致（星轨可视化、vibe 滤镜、语气滑杆）
- AI 功能更丰富（摘要 + 重写 + 心情播报）
- 版本迭代更快（0.6.10 → 0.7.25 仅用 4 天）
- 提交材料更完整（evidence、regression、validation）

**agentic-mail 优势**:
- Mail Host Service 由本项目自建（既用又建的反哺叙事）
- 行动卡片闭环（日程→日历、订单→追踪）
- 增量同步机制（保留已分析结果）
- 规则引擎离线回退（不依赖 LLM）

---

### 2.2 no-reminder-agent (nineanswerer)

| 维度 | no-reminder-agent 0.5.1 | agentic-mail 0.4.1 |
|------|------------------------|-------------------|
| **定位** | 邮件事项观察器 | 邮件行动平台 |
| **核心功能** | 检查 INBOX 最近邮件，识别事项，保存标题/截止/出处 | 邮件分类、行动建议、规则授权、闭环追踪 |
| **AI 使用** | model.complete 语义判断 | 规则引擎 + LLM（待验证） |
| **能力声明** | mail + model + storage | mail + storage |
| **限制** | 不发送邮件、不发提醒、应用关闭后不观察 | 功能更全面 |

**对比结论**: no-reminder-agent 是轻量级事项提取工具，功能范围远小于 agentic-mail。

---

## 三、其他赛道作品概览

### 3.1 日历应用 (com.oma.octosense.calendar)
- **版本**: 0.4.1
- **功能**: 实时天气、假期倒计时、四级优先级标记、ICS 导入
- **能力**: net + storage
- **特点**: 纯修复版本，专注稳定性

### 3.2 购物助手 (traceshop.shopping)
- **版本**: 0.4.0
- **功能**: 购物需求→提案→确认→本地草稿
- **能力**: octos.session.open + octos.turn.start + storage
- **特点**: 合成数据演示，无真实下单/支付/物流

### 3.3 记忆三件套 (os.memory + memory-notes + memory-digest)
- **版本**: 0.1.0
- **功能**: 跨应用共享记忆（系统应用 + 写入器 + 读取器）
- **能力**: storage + octos.turn.start
- **特点**: 三仓库联合提交，架构设计完整

### 3.4 观影筛选室 (oncue-screening-room)
- **版本**: 0.4.8
- **功能**: 未知（需进一步调研）

---

## 四、竞争力评估

### 4.1 我们的优势 ✅

1. **Mail Host Service 自建**
   - 既用又建的反哺叙事
   - 增量同步机制（保留已分析结果）
   - 时区转换、MIME 解析等底层能力

2. **行动卡片闭环**
   - 日程→日历添加
   - 订单→物流追踪
   - 变更→旧/新对比

3. **规则引擎离线回退**
   - 不依赖 LLM 也能工作
   - 关键词匹配 + 字段校验

4. **增量同步**
   - 只 fetch 新邮件（UID > last_synced_uid）
   - 保留已分析结果

### 4.2 我们的劣势 ❌

1. **UI 设计落后于 VibeMail**
   - 缺少可视化元素（星轨、vibe 滤镜）
   - 交互细节不足（语气滑杆、心情播报）

2. **AI 功能未闭环**
   - LLM 分析路径未验证通过
   - 缺少 AI 摘要、语气重写等高级功能

3. **版本迭代速度慢**
   - VibeMail 4 天迭代 2 个版本（0.6.10 → 0.7.25）
   - 我们 6 天迭代 4 个版本（0.1.0 → 0.4.1）

4. **提交材料不完整**
   - 缺少 evidence 目录
   - 缺少 regression 测试
   - 缺少 SUBMISSION.md 详细回答

### 4.3 竞争力评级

| 作品 | 竞争力 | 说明 |
|------|--------|------|
| **VibeMail** | ⭐⭐⭐⭐⭐ | 邮件赛道标杆，UI/AI/工程化全面领先 |
| **agentic-mail** | ⭐⭐⭐ | 功能全面但 UI/AI 落后，Mail Host Service 是亮点 |
| **no-reminder-agent** | ⭐⭐ | 轻量级工具，功能范围有限 |
| **com.oma.octosense.calendar** | ⭐⭐⭐ | 日历赛道领先，功能完整 |
| **traceshop.shopping** | ⭐⭐ | 购物赛道，合成数据演示 |
| **memory 三件套** | ⭐⭐⭐ | 架构设计完整，跨应用共享记忆 |

---

## 五、改进建议（10/9 复赛截止前）

### 5.1 高优先级（必须完成）

1. **LLM 分析路径闭环**
   - 解决环境变量传递问题
   - 验证 is_calendar/events/order/change 字段生成
   - 在 mail_cache.json 中确认 LLM 分析结果

2. **UI 打磨**
   - 任务驱动卡片流（差异化于 VibeMail 的三栏传统布局）
   - 行动建议可视化（批准/忽略按钮）
   - 规则授权界面

3. **提交材料完善**
   - 新建 evidence/ 目录
   - 编写 SUBMISSION.md（7 项问题详细回答）
   - 添加 regression 测试截图

### 5.2 中优先级（尽量完成）

1. **AI 分类 + 行动建议**
   - 日程邮件→添加日历按钮
   - 订单邮件→追踪卡片
   - 变更邮件→旧/新对比

2. **Demo 数据增强**
   - 自测 6/6 分析正确
   - 截图真实渲染

3. **README 重写**
   - 面向评委
   - 突出差异化（Mail Host Service 自建 + 行动卡片闭环）

### 5.3 低优先级（有时间再做）

1. **订单→追踪卡闭环**
2. **规则授权持久化**
3. **演示视频录制**

---

## 六、结论

**当前竞争力**: 中等偏上（⭐⭐⭐/⭐⭐⭐⭐⭐）

**核心优势**: Mail Host Service 自建（既用又建的反哺叙事）+ 行动卡片闭环

**核心劣势**: UI 设计落后于 VibeMail，AI 功能未闭环

**关键行动**: 10/9 前完成 LLM 路径闭环 + UI 打磨 + 提交材料完善，有望在复赛中脱颖而出。

**风险**: VibeMail 迭代速度极快（4 天 2 版本），可能继续拉开差距。

**建议策略**: 不模仿 VibeMail 的三栏传统布局，坚持任务驱动卡片流 + 决策型 AI 路线，突出差异化。
