<!--
Sync Impact Report
==================
Version change: 0.0.0 → 1.0.0 (初始制定)
Modified principles: 无（首次创建）
Added sections:
  - Core Principles (4 项原则)
  - 技术约束
  - 开发工作流
  - Governance
Removed sections: 无
Templates requiring updates:
  - .specify/templates/constitution-template.md ✅ 无需修改（模板文件）
  - .specify/templates/spec-template.md ✅ 无需修改
  - .specify/templates/plan-template.md ✅ 无需修改
  - .specify/templates/tasks-template.md ✅ 无需修改
Follow-up TODOs: 无
-->

# HDC Plus 项目准则

## 核心原则

### I. 中文优先（不可协商）

所有项目沟通、文档、代码注释、规格说明、提交信息 MUST 使用中文。

- 与 AI 助手的对话 MUST 使用中文。
- 所有文档（README、规格文档、计划文档、任务列表）MUST 以中文撰写。
- 所有代码中的注释 MUST 使用中文。
- 变量名、函数名、类型名等代码标识符 MUST 使用英文（遵循编程语言惯例）。
- Git 提交信息 MUST 使用中文。
- 对外 API 文档可提供中英双语版本，但中文版本 MUST 为默认版本。

**理由**：项目面向中文开发者社区，统一使用中文可降低协作门槛、提升沟通效率，并确保非核心开发人员（如产品经理、测试人员）能够无障碍参与项目讨论与文档评审。

### II. 规格驱动开发

所有功能 MUST 先有规格文档（spec.md），再进入规划与实现阶段。

- 每个功能分支 MUST 在 `specs/` 目录下有对应的规格子目录。
- 规格文档 MUST 包含：用户场景与优先级、验收场景、功能需求、成功标准。
- 实现之前 MUST 通过 checklist 验证规格完整性。
- 规格文档 MUST 面向非技术利益相关者编写，聚焦用户价值，不含实现细节。

**理由**：确保团队对功能需求有一致理解，避免在开发过程中因需求模糊导致的返工。

### III. 组件分离与独立可测

前后端（Rust / Vue）MUST 保持清晰边界，每个用户故事 MUST 可独立测试与交付。

- Tauri Rust 后端负责 HDC 交互、系统调用、进程管理。
- Vue 前端负责 UI 渲染、用户交互、状态管理。
- 前后端通过 Tauri IPC（invoke/command）通信，接口 MUST 有明确契约。
- 每个用户故事 MUST 可独立实现、独立测试、独立演示。

**理由**：组件分离确保各层可独立演进与测试。用户故事独立性确保 MVP 可增量交付，任何单一故事完成后即可产生可用价值。

### IV. 质量优先

代码质量与用户体验 MUST 优于交付速度。

- 关键用户路径（设备连接、安装、投屏）MUST 有明确的性能指标（见成功标准）。
- 所有用户可见的状态变化 MUST 有实时反馈（进度条、状态提示等）。
- 错误处理 MUST 对用户友好——用中文提示具体原因与建议操作。
- 边缘情况（设备断连、并发操作、异常退出）MUST 在设计阶段识别并覆盖。

**理由**：桌面应用的用户对稳定性与响应速度有较高期望，边缘情况处理不当会严重影响用户信任。

## 技术约束

- **语言/框架**: Rust (Tauri v2) + Vue 3 + TypeScript + Vite
- **包管理**: pnpm >= 9
- **目标平台**: Windows、macOS、Linux 桌面操作系统
- **外部依赖**: HDC（OpenHarmony Device Connector）工具，由系统环境提供，不在本项目内管理版本
- **代码标识符**: 变量、函数、类型、文件名 MUST 使用英文小驼峰/camelCase 或 snake_case 命名（遵循各语言惯例）
- **文档与注释**: MUST 使用中文

## 开发工作流

1. **规格阶段**: 编写 `spec.md`，通过 checklist 验证后进入计划阶段。
2. **计划阶段**: 产出 `plan.md`、`research.md`、`data-model.md`，通过准则检查后进入实现阶段。
3. **实现阶段**: 按用户故事优先级（P1 → P2 → P3）逐步实现，每个故事完成后独立验证。
4. **代码审查**: 每次代码合并前 MUST 检查是否符合本准则（中文注释、中文提交信息、规格对齐）。

## 治理

本准则是项目的最高约束性文件，所有开发实践 MUST 服从本准则。

- **修订流程**: 修订需提出变更提案，说明变更理由与影响范围，经项目负责人审批后方可生效。
- **版本管理**: 版本号遵循语义化版本规范（MAJOR.MINOR.PATCH）。
  - MAJOR: 原则删除或重大重新定义。
  - MINOR: 新增原则或章节、实质性扩展指导内容。
  - PATCH: 措辞澄清、笔误修复、非语义性调整。
- **合规检查**: 每次规格评审和代码审查 MUST 验证与本准则的一致性。

**Version**: 1.0.0 | **Ratified**: 2026-07-03 | **Last Amended**: 2026-07-03
