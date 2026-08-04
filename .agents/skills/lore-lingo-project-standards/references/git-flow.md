# Git Flow 与提交规范

LensLingo 采用 `main` 与 `develop` 双长期分支模型。`main` 始终对应可发布状态，`develop` 是所有常规开发工作的集成分支。两个长期分支都只能通过 PR 合并，禁止直接提交、直接推送、在本地直接合并后推送或强制推送。

本文中的“必须”“不得”是强制要求，“应该”是默认要求；偏离“应该”时需要在 PR 中说明理由。

## 目录

- [分支模型](#分支模型)
- [分支命名](#分支命名)
- [分支生命周期](#分支生命周期)
- [提交粒度](#提交粒度)
- [提交消息格式](#提交消息格式)
- [提交类型与作用域](#提交类型与作用域)
- [提交消息示例](#提交消息示例)
- [拉取请求与合并](#拉取请求与合并)
- [发布与热修复](#发布与热修复)
- [工作区与历史安全](#工作区与历史安全)

## 分支模型

```text
feature/* ─┐
fix/* ─────┼── PR ──> develop ── PR ──> main ──> 版本标签
docs/* ────┘

hotfix/* ──┬── PR ──> main
           └── PR ──> develop
```

### 长期分支

| 分支 | 用途 | 要求 |
| --- | --- | --- |
| `main` | 生产发布、版本标签和紧急修复基线 | 必须始终可构建、可测试、可发布；主要合并来源为 `develop` 和 `hotfix/*`；必须通过 PR 合并；禁止直接提交、直接推送、直接合并和强制推送 |
| `develop` | 常规开发集成和所有非热修复短期分支的基线 | 必须保持可构建、可测试；所有常规开发工作必须通过短期分支 PR 合入；禁止直接提交、直接推送、直接合并和强制推送 |

### 短期分支

| 前缀 | 用途 | 示例 |
| --- | --- | --- |
| `feature/` | 新增用户可见能力 | `feature/capture-history` |
| `fix/` | 修复非紧急缺陷 | `fix/subtitle-position` |
| `hotfix/` | 修复已发布版本的紧急生产问题 | `hotfix/capture-crash` |
| `refactor/` | 不改变外部行为的结构调整 | `refactor/desktop-adapters` |
| `perf/` | 可验证的性能改进 | `perf/frame-conversion` |
| `docs/` | 仅修改文档或规格 | `docs/git-flow` |
| `test/` | 仅增加或调整测试 | `test/capture-session` |
| `chore/` | 仓库维护、工具或非产品行为变更 | `chore/project-standards` |
| `build/` | 构建系统、依赖或打包变更 | `build/windows-installer` |
| `ci/` | CI/CD 配置与流水线变更 | `ci/macos-smoke-test` |

除 `hotfix/*` 外，所有短期分支必须从最新的 `develop` 创建，并只向 `develop` 提交 PR。`hotfix/*` 必须从最新的 `main` 创建，先向 `main` 提交 PR，再通过 PR 将相同修复同步到 `develop`。每个短期分支只承载一个明确目的，并在所有目标分支合并完成后删除。常规发布通过 `develop` 向 `main` 提交 PR，不维护常驻 `release` 分支。

Codex 创建的分支必须增加 `codex/` 前缀，格式为 `codex/<type>/<topic>`，例如 `codex/feature/capture-history`。该前缀只标识创建者，不改变分支用途或质量门槛。

## 分支命名

分支名必须满足以下规则：

- 使用小写 ASCII 字母、数字和连字符；不得使用空格、下划线或中文。
- `<type>` 必须来自短期分支表中的前缀。
- `<topic>` 应描述业务目标或修改对象，使用简短的 kebab-case，避免 `misc`、`update`、`changes` 等模糊名称。
- 关联工单时，可将编号放在主题开头，例如 `fix/123-subtitle-position`。
- 不得在同一分支混入不相关的重构、格式化或依赖升级。

## 分支生命周期

### 1. 创建

创建常规开发分支前必须确认工作区状态，并从远端以快进方式更新 `develop`：

```bash
git status --short --branch
git switch develop
git pull --ff-only origin develop
git switch -c feature/capture-history
```

Codex 创建分支时使用：

```bash
git switch -c codex/feature/capture-history
```

创建紧急修复分支时改为从最新的 `main` 开始：

```bash
git status --short --branch
git switch main
git pull --ff-only origin main
git switch -c hotfix/capture-crash
```

工作区存在未提交内容时，不得切换分支，除非已确认这些内容属于目标分支且切换不会覆盖或混入其他用户的工作。

### 2. 开发与同步

- 小步提交，每个提交都必须保持内部一致并可独立审查。
- 提交前仅暂存当前提交所需文件，使用 `git diff --cached` 检查实际提交内容。
- 尚未共享的常规开发分支可以通过 `git rebase origin/develop` 保持线性历史；热修复分支使用 `origin/main`。
- 已被他人使用的共享分支不得通过 rebase 改写历史；通过合并目标分支同步，或协调后再改写。
- 解决冲突后必须重新运行受影响范围的检查。

### 3. 提交 PR

- 将常规开发短期分支推送到远端，并向 `develop` 创建 PR；不得直接向 `main` 提交常规开发 PR。
- 常规发布由 `develop` 向 `main` 创建 PR；紧急修复由 `hotfix/*` 向 `main` 创建 PR。
- PR 必须通过适用的构建、测试、静态检查和平台验证。
- PR 合并前必须处理高严重性审查意见，并确认分支未落后于目标分支的必要变更。

### 4. 合并与清理

- 常规短期分支合入 `develop` 时默认使用 squash merge，使每个提交对应一个完整、可回滚的变更。Squash 后的提交消息必须符合本文的提交消息格式。
- `develop` 合入 `main` 时必须使用合并提交，不得 squash 或 rebase，以保留两个长期分支的共同祖先并避免后续发布 PR 重复包含历史变更。
- `hotfix/*` 合入 `main` 以及同步到 `develop` 时都必须使用合并提交，不得 squash 或 rebase，以保留同一修复提交的祖先关系；不得在两个目标分支分别生成内容相同但历史无关的提交。
- 仅当保留多提交历史具有明确审查或迁移价值时，常规短期分支才使用普通 merge，并在 PR 中说明原因。
- 短期分支完成所有必需 PR 合并后，删除本地和远端分支；不得继续在已合并分支上开发新需求。

## 提交粒度

一个提交必须表达一个完整意图，并满足以下要求：

- 代码、测试、文档、OpenSpec 产物和数据库迁移可以在共同实现同一行为时一起提交。
- 不相关的功能、修复、重构、格式化、依赖升级和生成文件必须拆分提交。
- 修复缺陷时应在同一提交中包含能够防止回归的测试；缺少测试基础设施时在 PR 中明确记录。
- 数据库迁移必须与依赖该迁移的代码和验证保持可审查的一致性，不得修改已发布迁移。
- 每个提交都应能够构建；不得故意提交只在后续提交中才恢复可构建状态的中间步骤。
- 禁止提交密钥、令牌、个人数据、构建产物、IDE 状态或操作系统生成文件。

## 提交消息格式

提交消息遵循 Conventional Commits，完整格式如下：

```text
<type>[(<scope>)][!]: <summary>

<body>

<footer>
```

其中方括号表示可选项，不是提交消息的字面字符。只有 `<type>` 和 `<summary>` 必填；`(<scope>)`、`!`、正文和脚注按需使用。

### 标题

- 格式必须为 `<type>[(<scope>)][!]: <summary>`；无作用域时使用 `<type>: <summary>`，破坏性变更使用 `<type>(<scope>)!: <summary>` 或 `<type>!: <summary>`。
- `<type>` 和 `<scope>` 必须使用小写 ASCII；冒号后必须有一个空格。
- `<summary>` 使用英文祈使语气和现在时，例如 `add`、`fix`、`remove`，不得使用 `added`、`fixed`。
- `<summary>` 首字母小写，不使用句号结尾，不写工单编号，不使用含义模糊的 `update code` 或 `fix bug`。
- 整个标题必须控制在 72 个字符以内；无法写清时优先缩小提交范围。

### 正文

正文用于解释“为什么修改”和“行为有何变化”，而不是逐行复述代码：

- 标题与正文之间必须空一行。
- 使用完整句子，可分段或使用短列表。
- 说明关键约束、取舍、兼容性和无法从差异直接看出的实现原因。
- 单行建议不超过 100 个字符。
- 简单且自解释的提交可以省略正文。

### 脚注

脚注与正文之间必须空一行，用于记录关联事项和破坏性变更：

```text
Closes: #123
Refs: #456
BREAKING CHANGE: subtitle events now use the subtitle prefix.
```

存在破坏性变更时，必须在类型或作用域后增加 `!`，并使用 `BREAKING CHANGE:` 脚注说明迁移方式。仅添加 `!` 而没有迁移说明不合格。

## 提交类型与作用域

### 类型

| 类型 | 使用场景 | SemVer 影响 |
| --- | --- | --- |
| `feat` | 新增或扩展用户可见能力 | 默认 MINOR |
| `fix` | 修复缺陷或错误行为 | 默认 PATCH |
| `perf` | 不改变功能语义的性能优化 | 默认 PATCH |
| `refactor` | 不新增功能且不修复缺陷的代码重构 | 无直接影响 |
| `test` | 仅修改测试、夹具或测试基础设施 | 无直接影响 |
| `docs` | 仅修改文档、注释或规格说明 | 无直接影响 |
| `build` | 构建系统、依赖、打包或安装器变更 | 视行为而定 |
| `ci` | CI/CD 工作流与自动化变更 | 无直接影响 |
| `chore` | 无法归入以上类型的仓库维护工作 | 无直接影响 |

任意类型带有 `!` 或 `BREAKING CHANGE:` 时均表示 SemVer MAJOR 变更。版本影响最终以产品发布策略和 PR 说明为准。

### 作用域

作用域是受影响的稳定模块、平台或能力边界，应该使用仓库中可识别的短名称。推荐作用域包括但不限于：

```text
ui, capture, subtitle, settings, composable, ipc, desktop, db,
migration, windows, macos, deps, release, openspec, skill
```

- 修改集中在一个边界时应该填写作用域，例如 `fix(windows)`。
- 跨越多个紧密相关边界且无法用一个准确名称概括时可以省略作用域。
- 不得使用文件名、人员名、工单编号或 `misc` 作为作用域。
- 同一概念应沿用已有作用域，避免在 `db`、`database`、`storage` 等近义词之间随意切换。

## 提交消息示例

简单功能：

```text
feat(capture): add per-display capture sessions
```

缺陷修复：

```text
fix(windows): release GDI handles after capture
```

带正文和关联工单：

```text
fix(subtitle): restore the saved position after display changes

Recalculate the visible bounds before applying the persisted position so the
The Subtitle Overlay remains reachable when a monitor is disconnected.

Closes: #128
```

破坏性变更：

```text
refactor(subtitle)!: standardize subtitle events

Use the product-standard subtitle prefix across the IPC boundary.

BREAKING CHANGE: consumers must subscribe to subtitle events.
```

不合格示例：

```text
update code
fix: fixed bug.
feat(ui): add settings and refactor database and update deps
```

这些消息分别存在类型缺失、时态和标点错误、以及提交范围混杂的问题。

## 拉取请求与合并

- 常规开发 PR 的目标分支必须是 `develop`；`main` 的常规合并来源必须是 `develop`，紧急修复来源必须是 `hotfix/*`。其他来源需要明确说明例外原因并获得维护者批准。
- `develop` 和 `main` 都必须启用分支保护，至少要求 PR、必要状态检查和审查通过，并禁止直接推送、强制推送和删除。
- PR 标题必须符合提交消息标题格式；常规短期分支默认 squash merge 时，该标题会成为 `develop` 上的提交标题。
- PR 描述必须说明问题、行为变化、架构影响、验证结果、平台覆盖和已知限制。
- 行为或架构由 OpenSpec 定义时，必须关联对应变更；数据库变更必须说明迁移与回滚策略。
- 平台代码必须通过对应 Windows/macOS 原生 CI；数据库变更必须通过迁移检查。
- 不得合并存在失败的必需检查、未解决的高严重性意见或未说明的破坏性变更的 PR。
- 审查期间优先使用新提交修复意见；不得为了“整理历史”改写其他人正在使用的分支。
- 禁止通过本地合并后推送的方式绕过 `develop` 或 `main` 的 PR 审查流程。

## 发布与热修复

- 常规发布必须由 `develop` 向 `main` 创建发布 PR；通过所需检查和冒烟测试后，使用合并提交完成合并。
- 仅从通过所需检查和冒烟测试的 `main` 创建发布产物。
- 使用 SemVer 标签 `vMAJOR.MINOR.PATCH`，例如 `v1.4.2`；标签必须指向 `main` 上的提交。
- 发布前同步 `package.json`、Cargo 和 Tauri 配置中的版本，并生成包含用户可见变更、迁移、平台支持和已知限制的发布说明。
- 不得提交签名凭据、证书私钥或生成的安装包。
- 紧急生产缺陷从最新 `main` 创建 `hotfix/<topic>`，通过 PR 合入 `main`，并执行与普通修复相同的审查和 CI 门槛。
- 热修复合入 `main` 后，必须在删除 `hotfix/*` 前通过 PR 将同一修复同步到 `develop`；两个长期分支都包含该修复后才能发布新的 PATCH 版本并清理分支。
- 若未来确实需要发布冻结期或多版本并行维护，必须先更新本规范，明确 `release/*` 或维护分支的来源、回合并策略、负责人和终止条件。

## 工作区与历史安全

- 在编辑、暂存、提交、切换分支、rebase 或合并前检查 `git status`。
- 提交前检查 `git diff --cached --check` 和完整的 `git diff --cached`。
- 仅暂存当前意图所属文件；保留用户拥有的、未跟踪的和不相关的工作区改动。
- 提交前运行与改动风险相匹配的测试和检查，并在 PR 中如实记录结果与缺口。
- 不得使用 `git reset --hard`、强制 checkout 或清理命令处理来源不明的改动。
- 不得改写共享历史或 force-push；确需执行时必须获得明确授权并先确认影响范围。
- 不得在未检查目标和提交内容的情况下执行批量 `git add .` 或 `git add -A`。
