## ADDED Requirements

### Requirement: Explicit single-shot capture translation
The system SHALL start capture translation only after an explicit user action and SHALL own at most one active single-shot task.

#### Scenario: User starts a new task
- **WHEN** the user starts capture translation while no task is active
- **THEN** the system captures selector backgrounds before showing temporary selector windows and reports an active task identifier

#### Scenario: User replaces an active task
- **WHEN** the user starts a new capture translation while another task is active
- **THEN** the system cancels and cleans up the previous task before activating the replacement

#### Scenario: User cancels selection
- **WHEN** the user presses Escape or otherwise cancels the selector
- **THEN** every selector window is destroyed, transient images are released, and the main window is restored without reporting success

### Requirement: Monitor-local region selection
The system SHALL represent a selected region as a monitor identifier plus a monitor-local logical rectangle and SHALL crop the pre-captured image using the selected monitor scale.

#### Scenario: Region is selected on a Retina display
- **WHEN** the selector returns a valid logical rectangle on a scaled display
- **THEN** the system converts its start with bounded floor rounding, its end with bounded ceil rounding, and crops the matching physical pixels

#### Scenario: Region is invalid
- **WHEN** the selection has no area, exceeds the owning monitor, or exceeds configured image bounds
- **THEN** the system rejects it with `invalid_region` and cleans up the selector task

### Requirement: Structured OCR layout document
The system SHALL represent recognized content as a layout document containing image dimensions and ordered text blocks with stable IDs, normalized top-left-origin bounds, source text, block type, and confidence.

#### Scenario: Text is recognized
- **WHEN** OCR returns one or more valid text observations
- **THEN** the system normalizes their bounds, assigns stable IDs and reading order, and retains their spatial separation in the layout document

#### Scenario: No text is recognized
- **WHEN** OCR succeeds but returns no usable observations
- **THEN** the task terminates with an explicit empty result and does not call the translation provider

#### Scenario: OCR backend is unavailable
- **WHEN** capture translation runs on a platform without a production OCR adapter
- **THEN** the system returns `unsupported_capability` and does not synthesize OCR content

### Requirement: ID-preserving structured translation
The system MUST translate bounded groups of layout blocks while preserving every input block ID and MUST reject incomplete, duplicated, or unknown output IDs.

#### Scenario: Translation succeeds
- **WHEN** the provider returns exactly one valid translation for every submitted block ID
- **THEN** the system attaches each translated text to the matching layout block without changing geometry or reading order

#### Scenario: Translation configuration is missing
- **WHEN** a required endpoint, model, or remote API key is absent
- **THEN** the system returns `translation_not_configured` before sending recognized text to a network destination

#### Scenario: Provider response cannot be mapped safely
- **WHEN** the provider omits an ID, duplicates an ID, adds an unknown ID, or returns an oversized response
- **THEN** the system returns `translation_protocol_error` and does not display partially mapped translations as complete

### Requirement: Layout-preserving result presentation
The application SHALL present the captured image at a stable aspect ratio and SHALL position translated text from normalized block geometry without changing block identity or reading order.

#### Scenario: User views translated mode
- **WHEN** a completed layout document is shown in translated mode
- **THEN** each translated block is rendered over its original region with a readable surface, bounded text sizing, and no change to surrounding image geometry

#### Scenario: User changes result mode
- **WHEN** the user switches among original, translated, and bilingual modes
- **THEN** the same document and geometry are reused without rerunning OCR or translation

#### Scenario: Translation is longer than the source region
- **WHEN** translated text cannot fit at the preferred size
- **THEN** the block reduces text size within configured bounds, wraps text, and contains any remaining overflow without resizing or overlapping adjacent blocks

### Requirement: Private and recoverable task lifecycle
系统 MUST 不将截图、识别文本和译文写入浏览器或应用持久化存储；API 凭据 MUST 仅持久化到原生系统凭据存储，且系统 SHALL 提供确定的任务状态和安全错误。

#### Scenario: Task reaches a terminal state
- **WHEN** a task succeeds, fails, or is cancelled
- **THEN** native selectors and workers are cleaned up, the main window is restored, and the frontend reaches exactly one terminal state

#### Scenario: Diagnostics are emitted
- **WHEN** OCR or translation fails
- **THEN** user-facing IPC omits API credentials, authorization headers, captured text, native handles, local paths, and raw provider responses

#### Scenario: User retries after failure
- **WHEN** the user retries a recoverable failure
- **THEN** the system starts a new task using current persisted configuration and does not reuse stale selector or result state

### Requirement: 可校验的持久化翻译设置
设置页 SHALL 读取并保存区域和窗口任务共用的持久化配置；保存时 SHALL 使用实际翻译请求相同的端点与凭据规则，不因配置通过校验而声称已验证服务连通性。

#### Scenario: 应用有效配置
- **WHEN** 用户显式应用包含有效地址、模型和所需 API Key 的配置
- **THEN** 系统去除字段首尾空白并以同一 SQLite 事务保存地址、模型和默认语言，密钥仅写入系统凭据存储，数据库只保留引用，下一次捕获与重启后使用该配置
- **AND** IPv4 与 IPv6 回环服务可以不填写 API Key

#### Scenario: 拒绝无效配置
- **WHEN** 地址无效、包含 URL 凭据、查询参数或片段、违反已有网络地址限制，或者远程服务缺少 API Key
- **THEN** 系统拒绝保存并保留上一次已应用的配置，设置页显示安全的错误提示

#### Scenario: 加载失败与修改草稿
- **WHEN** 配置读取失败
- **THEN** 设置页禁止覆盖保存并提供重新读取入口
- **WHEN** 用户修改配置后选择撤销修改
- **THEN** 设置页恢复最近一次已读取或已应用的值，不发起保存

#### Scenario: 说明尚未接入的设置
- **WHEN** 用户查看设置页
- **THEN** 当前可用配置优先展示，自动翻译、历史与字幕库的未完成状态明确标注
- **AND** 未完成配置不显示为当前单次捕获已生效的选项，更多功能支持键盘展开与收起

#### Scenario: 重启恢复配置
- **WHEN** 应用重新启动并读取配置
- **THEN** 系统从 SQLite 恢复默认模型、地址和语言，从原生凭据存储恢复 API Key，不依赖先前进程或浏览器存储

#### Scenario: 存储失败
- **WHEN** 凭据写入或数据库事务失败
- **THEN** 系统返回安全的结构化错误，保留上一次已提交的配置和密钥，不显示保存成功
- **AND** 新建但未提交的凭据被尽力清理，绝不降级为明文存储

#### Scenario: 系统凭据丢失或不可访问
- **WHEN** 已保存的凭据条目被外部删除
- **THEN** 系统恢复非密钥配置并将密钥字段留空，允许用户重新填写保存
- **WHEN** 系统拒绝凭据读取
- **THEN** 配置读取失败且设置页禁用覆盖保存，提供重试入口
