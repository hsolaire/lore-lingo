## 1. Contracts and domain behavior

- [x] 1.1 Add serializable layout, selection, translation configuration, task state, and structured error models in Rust and TypeScript
- [x] 1.2 Implement and test normalized geometry validation, monitor-scale crop conversion, reading-order sorting, stable block IDs, and basic block classification

## 2. OCR and translation adapters

- [x] 2.1 Add the target-gated macOS Vision OCR adapter and explicit unsupported adapter for other platforms
- [x] 2.2 Add the bounded OpenAI-compatible translation client with endpoint, credential, response-size, and exact-ID validation
- [x] 2.3 Remove the real-looking default API key and expose in-memory translation configuration to the capture workflow

## 3. Capture translation service

- [x] 3.1 Implement the single-active-task service that pre-captures monitor images, owns transient configuration, and cleans up every terminal path
- [x] 3.2 Add narrow Tauri commands for starting, reading selector payload, completing, and cancelling capture translation
- [x] 3.3 Create temporary per-monitor selector windows before selection and restore the main window on completion, failure, or cancellation

## 4. Frontend workflow and presentation

- [x] 4.1 Add a typed frontend service and composable with deterministic idle, selecting, processing, empty, success, error, and cancelled states
- [x] 4.2 Add the full-window selector entry with frozen background, pointer selection, dimensions, Escape cancellation, and keyboard-safe cleanup
- [x] 4.3 Add the layout-preserving result workspace with original, translated, and bilingual modes plus bounded responsive text blocks
- [x] 4.4 Connect the custom-region action to the real workflow and remove the timer-based simulated capture success

## 5. Verification

- [x] 5.1 Add focused tests for DTO validation, coordinate conversion, translation ID mapping, empty OCR, cancellation, and stale task handling
- [x] 5.2 Run frontend build, Rust format/check/test, browser visual checks, and a native macOS capture/OCR smoke test; document any platform gaps

## 验证记录

- `pnpm build` 与 `pnpm test` 通过；前端共 13 个测试文件、42 项测试。
- `cargo fmt --check`、`cargo check`、`cargo test` 与严格 Clippy 检查通过；常规 Rust 测试包含 28 项通过、1 项显式原生烟雾测试忽略。
- macOS Vision 空白 PNG OCR 烟雾测试已在非沙箱环境使用 `--ignored --exact` 单独运行并通过；受限沙箱无法调用该系统框架。
- 原生 Tauri 开发版成功启动。由于本机同时运行已安装版 LoreLingo，自动化无法按相同 bundle id 区分两个实例；选择器窗口的真实拖拽仍需人工在当前开发版窗口确认。
- 浏览器预览只能验证显式后台不可用状态，无法代替 Tauri IPC、原生权限、临时窗口层级和显示器坐标验收。

## 2026-10-10 四入口与原生捕获复核

- 顶部入口收敛为翻译选项、捕获窗口、捕获区域、设置；复制、版面与字幕显示模式位于翻译选项，模型与服务配置位于独立设置页。
- `pnpm test`：17 个测试文件、59 项通过；`pnpm build` 通过。新增窗口选择触发捕获、Escape 焦点恢复、共享配置与失败保留缩略图测试。
- `cargo fmt`、严格 Clippy、`cargo test` 和原生 arm64 `cargo build` 通过；32 项常规 Rust 测试通过，1 项 Vision 原生烟雾测试默认忽略；目录解析新增过期与未知来源断言。
- 在 macOS 27.0.1（26A434）、arm64、单 Retina 显示器上验证真实 ScreenCaptureKit 显示器冻结画面、区域拖拽、OCR 后未配置翻译错误、已捕获状态、真实区域缩略图，以及系统设置窗口的真实捕获。
- 录屏授权实际启用并经重启验证；发现临时预览标识复用旧代码签名，改为忽略的稳定构建目录、完整本地签名与仅针对开发标识的重新授权。最终预览位于 `target/toolbar-preview/LoreLingo.app`，没有发布或正式签名公证。
- 原生下拉的原文、译文、双语选项均完整显示，独立设置页读取共用进程内服务配置；未提供实际 endpoint、模型或 API key，因此未声称远程翻译成功。
- Windows、Intel、多显示器、macOS 13、连续捕获、正式签名公证及完整辅助功能矩阵仍未执行；进程内服务配置不跨重启保存。

## 2026-10-10 区域实时预览与坐标验证

- 增加区域单帧轮询、停止与过期响应丢弃、坐标更新与非法输入的前端测试；补充 Escape 回归后完整执行 `pnpm test`，19 个测试文件、65 项全部通过。前端构建通过。
- Rust 常规测试 33 项通过、1 项 Vision 测试默认忽略；`cargo check`、严格 Clippy、`cargo build`、格式检查与 `git diff --check` 通过。
- macOS arm64 单 Retina 原生验证：本地时钟从 11:29:24 到 11:29:46 持续变化，预览排除 LoreLingo 自身窗口；X1/X2/Y1/Y2 从 85/760/350/505 改为 100/700/360/490，物理截图从 1350×310 变为 1200×260；X2=50 时禁用应用并提示错误，恢复 700 后可重新捕获。
- “翻译此区域”实际截取新区域，版面中显示新的 11:30:05 时钟；远程服务未配置，明确返回 `translation_not_configured`，未声称译文成功。关闭再打开区域面板保留已应用坐标，输入框内 Escape 关闭面板并返回入口焦点。
- 仅更新已授权的本机开发预览；原生预览仍为单帧有界轮询，关闭面板停止后续捕获，未实现长期连续 OCR/翻译。Windows、Intel、外接或多显示器未验证。
- 正式发布预检仍存在既有阻碍：正式标识以 `.app` 结尾、生产 CSP 为空、缺少明确最低系统版本、额外 package-lock.json、工作区未提交；本次没有发布安装器，也没有变更这些发布配置。
