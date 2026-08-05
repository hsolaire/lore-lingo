---
name: lore-lingo-project-standards
description: 统一执行 LoreLingo 仓库的项目结构、Vue/TypeScript 与 Naive UI 前端、macOS 27 与 Windows 11 界面、Rust/Tauri 后端、SQLite 迁移、Git Flow、测试、安全、持续集成、打包和发布工程规范。Codex 在 lore-lingo 仓库中规划、实现、审查、测试、提交或发布变更时使用。
---

# LoreLingo 项目工程规范

每次修改仓库时都必须遵守这些规范。将现有源代码和已接受的 OpenSpec 产物视为当前架构；已有明确边界时，不得另行设计平行结构。

## 工作流程

1. 检查需求范围、相关源文件、进行中的 OpenSpec 变更和工作区状态。
2. 新增或移动文件前，必须阅读 `references/directory-structure.md`。
3. 仅加载下方路由表中与任务相关的参考文档。
4. 仓库尚未明确某项决策时，说明关键假设。
5. 按所选规范实现最小且完整的变更。
6. 根据风险程度运行 `references/testing-quality.md` 中要求的检查。
7. 创建分支、提交、拉取请求、标签或发布前，重新阅读 `references/git-flow.md`。

## 参考文档路由

| 任务范围 | 必读参考文档 |
| --- | --- |
| 任何源代码或目录变更 | `directory-structure.md`、`testing-quality.md` |
| Vue、TypeScript、CSS、组件、组合式函数、Tauri 前端 API | `frontend.md` |
| Naive UI 组件、提供器、主题、图标或前端依赖 | `frontend.md`、`frontend-naive-ui.md`、`security.md` |
| macOS 窗口呈现、Liquid Glass、平台主题或原生材质 | `frontend.md`、`frontend-naive-ui.md`、`frontend-macos-27.md`、`backend.md` |
| Windows 窗口呈现、Fluent 2、Mica/Acrylic 或平台主题 | `frontend.md`、`frontend-naive-ui.md`、`frontend-windows-11.md`、`backend.md` |
| 前端视觉实现或审查 | `frontend-visual-validation.md`、`testing-quality.md`，以及适用的平台参考文档 |
| Rust、Tauri 命令、原生平台 API、服务 | `backend.md` |
| SQLite 模式、查询、持久化、迁移 | `database.md`、`backend.md` |
| 分支、提交、拉取请求、标签 | `git-flow.md` |
| 依赖、凭据、能力、外部输入 | `security.md` |
| 持续集成、安装器、签名、版本、分发 | `release.md`、`git-flow.md`、`security.md` |
| 跨领域架构变更 | 所有相关参考文档，以及进行中的 OpenSpec 提案、设计和规格 |

必须直接读取本 Skill 的 `references/` 目录中的参考文档。修改受规范约束的代码时，不得依赖记忆中的摘要。

## 全局不变量

- 保留用户拥有的改动和工作区中与当前任务无关的改动。
- 所有面向用户的输出和仓库文档必须使用简体中文。`.agents/` 下的 Skill 描述、指令、参考文档和界面元数据等自然语言内容必须使用简体中文。README 文件可以同时使用简体中文和英文。代码标识符、协议字段、命令、第三方 API 字面量及其他机器定义文本应保留其要求的语言。
- 产品术语统一使用“悬浮字幕”（`Subtitle Overlay`）。PascalCase 标识使用 `Subtitle`，驼峰标识、路径、事件和 Tauri 窗口标签使用 `subtitle`。
- Vue 中不得出现原生句柄、原始 SQL 或平台框架类型。
- Tauri 命令处理器必须保持精简；领域行为放在 Rust 服务中，操作系统行为放在平台适配器中。
- 不得在源代码、日志、`localStorage`、SQLite、测试夹具或截图中存储明文密钥。
- 不得将生成输出、构建产物、集成开发环境状态或操作系统文件提交到 Git。
- 跨进程边界使用结构化错误，避免泄露原生路径或句柄。
- 为行为变更新增或更新测试；不得把模拟成功描述为已经实现的生产行为。
- 行为、能力合约或架构变化时，更新相关 OpenSpec 产物。
- 优先返回明确的“不支持”状态，不得通过静默降级声称成功。

## 冲突解决顺序

按以下顺序应用规则：

1. 用户请求和仓库内的 `AGENTS.md` 指令。
2. 已接受的 OpenSpec 能力规格和进行中的变更设计。
3. 本 Skill 及其参考文档。
4. 未受影响代码中的现有本地模式。

同一层级的两个来源发生冲突时，停止受影响的变更，列明两项规则，并在实现前解决冲突。

## 维护本 Skill

- `SKILL.md` 只保留路由和不变量。
- 详细策略放在单层 `references/` 文件中，避免重复规则。
- 建立新的仓库级约定时，在同一变更中更新相应参考文档。
- 编辑后使用 `skill-creator` 的快速验证器校验本 Skill。
