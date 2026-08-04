# 目录结构

## 目的

通过路径清晰体现归属边界。新文件必须遵循此目标结构；对现有文件进行完整一致的变更时，可以逐步迁移。

## 仓库布局

```text
lore-lingo/
  .agents/skills/               项目级 Codex Skill
  .github/workflows/            持续集成与发布自动化
  docs/                         长期维护的架构与运维文档
  openspec/                     能力规格与变更产物
  prototype/                    仅存放非生产视觉原型
  src/                          Vue/TypeScript 前端
  src-tauri/                    Rust/Tauri 桌面后端
  package.json                  前端脚本与依赖
  pnpm-lock.yaml                唯一的 JavaScript 锁文件
```

## 前端布局

```text
src/
  components/
    ui/                         可复用视觉基础组件
    main-panel/                 主窗口功能组件
    subtitle/                   悬浮字幕组件
    settings/                   设置功能组件
  composables/                  可复用响应式编排
  services/                     类型化 Tauri/API 客户端；按需添加
  types/                        共享前端合约类型；按需添加
  icons/                        图标注册表
  styles/                       全局设计令牌与跨窗口样式
  App.vue                       主窗口组合根组件
  SubtitleApp.vue               悬浮字幕组合根组件
  main.ts                       主入口
  subtitle.ts                   悬浮字幕入口
```

规则：

- 组件负责渲染和交互，不负责原生集成或持久化。
- 组合式函数负责 Vue 状态和生命周期编排，不负责原始平台细节。
- 随着代码库增长，所有直接调用 Tauri `invoke` 的代码都应该移到 `src/services/` 的类型化模块之后。
- 共享传输类型放在 `src/types/`；不得在多个组件中重复定义同一命令载荷。
- 生产入口不得导入 `prototype/`。

## 后端布局

```text
src-tauri/
  capabilities/                 Tauri 能力声明
  migrations/                   仅追加的 SQLite 迁移
  src/
    commands/                   精简的 IPC 处理器；按领域添加
    db/                         连接、迁移与仓储
    desktop/                    桌面能力门面与捕获会话
      platform/                 受目标平台约束的 Windows/macOS 适配器
    domain/                     稳定的业务类型与规则
    services/                   用例编排
    lib.rs                      应用组合与注册
    main.rs                     仅作为二进制入口
  Cargo.toml
  tauri.conf.json
```

规则：

- `lib.rs` 和命令模块不得堆积领域算法。
- 目标平台特定依赖和不安全代码必须保留在对应平台模块下。
- 数据库查询必须保留在 `db/` 仓储或职责明确的数据库服务中。
- 命令必须暴露用例，绝不能暴露通用 SQL、表名、文件系统路径或原生句柄。

## 命名

- Vue 组件：`PascalCase.vue`。
- TypeScript 模块和组合式函数：沿用现有驼峰命名约定，组合式函数以 `use` 开头。
- Rust 模块和文件：`snake_case`。
- OpenSpec 能力和 Skill 目录：`kebab-case`。
- 悬浮字幕标识：PascalCase 使用 `Subtitle`，驼峰标识、路径、事件和 Tauri 窗口标签使用 `subtitle`。
- SQLite 迁移：补零序号加下划线式描述，例如 `0002_add_capture_profiles.sql`。
- 测试：映射源代码归属边界，并使用测试框架的标准后缀。

## 放置决策

创建文件前，先确认其所有者和使用方。如果一个文件服务于多个无关领域，应拆分职责，不得创建通用的 `utils` 堆放目录。
