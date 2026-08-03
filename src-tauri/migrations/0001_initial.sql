PRAGMA foreign_keys = ON;

BEGIN IMMEDIATE;

-- =============================================================================
-- DB: database metadata
-- =============================================================================
CREATE TABLE db_schema_migrations ( -- 表：记录已成功执行的数据库结构迁移，作用类似 Flyway schema history。
    version     INTEGER PRIMARY KEY CHECK (version > 0), -- 迁移版本号；对应文件名前缀，例如 1 对应 0001_initial.sql。
    name        TEXT NOT NULL UNIQUE CHECK (length(trim(name)) > 0), -- 迁移名称，例如 initial、add_history_tags。
    applied_at  INTEGER NOT NULL DEFAULT (CAST(strftime('%s', 'now') AS INTEGER) * 1000) CHECK (applied_at >= 0) -- 迁移完成时间；Unix 毫秒时间戳。
) STRICT;

-- =============================================================================
-- SYS: local application configuration
-- =============================================================================
-- Extensible UI/runtime preferences only. Business data belongs in typed tables.
CREATE TABLE sys_app_preferences ( -- 表：保存不值得单独建列的低频应用偏好；核心业务配置不得放入此表。
    key         TEXT PRIMARY KEY, -- 偏好键；使用命名空间形式，例如 ui.theme、history.page_size。
    value_json  TEXT NOT NULL CHECK (json_valid(value_json)), -- JSON 任意值；示例："dark"、20、true、{"compact":true}。
    updated_at  INTEGER NOT NULL DEFAULT (CAST(strftime('%s', 'now') AS INTEGER) * 1000) -- 最后更新时间；Unix 毫秒时间戳。
) STRICT, WITHOUT ROWID;

CREATE TABLE sys_model_profiles ( -- 表：保存可切换的翻译模型、服务商端点和非敏感调用参数。
    id              INTEGER PRIMARY KEY, -- 模型配置本地整数主键。
    name            TEXT NOT NULL COLLATE NOCASE UNIQUE, -- 用户可读配置名称；忽略大小写且不可重复。
    provider        TEXT NOT NULL, -- 服务商标识，例如 openai、anthropic、deepl、ollama、openai_compatible。
    model_name      TEXT NOT NULL, -- 服务商模型标识，例如 gpt-4o-mini、qwen2.5:7b。
    api_base        TEXT, -- API 基础地址；使用服务商默认地址时可为空。
    secret_ref      TEXT, -- 系统钥匙串条目引用；API Key 本体禁止写入 SQLite。
    options_json    TEXT NOT NULL DEFAULT '{}' CHECK (json_valid(options_json) AND json_type(options_json) = 'object'), -- JSON 对象：{"temperature":0.2,"top_p":1.0,"max_tokens":1024,"extra_headers":{}}；不得含鉴权信息。
    is_enabled      INTEGER NOT NULL DEFAULT 1 CHECK (is_enabled IN (0, 1)), -- 启用状态：0=禁用，1=启用。
    created_at      INTEGER NOT NULL DEFAULT (CAST(strftime('%s', 'now') AS INTEGER) * 1000), -- 创建时间；Unix 毫秒时间戳。
    updated_at      INTEGER NOT NULL DEFAULT (CAST(strftime('%s', 'now') AS INTEGER) * 1000) -- 最后更新时间；Unix 毫秒时间戳。
) STRICT;

CREATE TABLE sys_translation_settings ( -- 表：保存全局唯一的当前翻译行为设置。
    id                      INTEGER PRIMARY KEY CHECK (id = 1), -- 单例主键；固定为 1。
    default_model_profile_id INTEGER REFERENCES sys_model_profiles(id) ON DELETE SET NULL, -- 默认模型配置；模型被删除后置空。
    mode                    TEXT NOT NULL DEFAULT 'auto' CHECK (mode IN ('manual', 'auto')), -- 翻译模式：manual=快捷键单次翻译，auto=按间隔持续翻译。
    auto_interval_ms        INTEGER NOT NULL DEFAULT 1000 CHECK (auto_interval_ms BETWEEN 500 AND 10000), -- 自动翻译间隔；500～10000 毫秒。
    source_language         TEXT NOT NULL DEFAULT 'en', -- 源语言 BCP 47 标识；自动检测使用 auto。
    target_language         TEXT NOT NULL DEFAULT 'zh-CN', -- 目标语言 BCP 47 标识，例如 zh-CN、en、ja。
    request_timeout_ms      INTEGER NOT NULL DEFAULT 30000 CHECK (request_timeout_ms BETWEEN 1000 AND 300000), -- 单次模型请求超时；1000～300000 毫秒。
    prefer_subtitles        INTEGER NOT NULL DEFAULT 1 CHECK (prefer_subtitles IN (0, 1)), -- 字幕优先：0=直接使用模型，1=先匹配字幕库。
    save_history            INTEGER NOT NULL DEFAULT 1 CHECK (save_history IN (0, 1)), -- 历史保存：0=不保存，1=保存到 tr_history。
    history_retention_days  INTEGER CHECK (history_retention_days IS NULL OR history_retention_days >= 1), -- 非收藏历史保留天数；NULL=永久保留。
    updated_at              INTEGER NOT NULL DEFAULT (CAST(strftime('%s', 'now') AS INTEGER) * 1000) -- 最后更新时间；Unix 毫秒时间戳。
) STRICT;

INSERT INTO sys_translation_settings(id) VALUES (1);

CREATE TABLE sys_ocr_settings ( -- 表：保存全局唯一的 OCR 引擎、采样和识别参数。
    id                  INTEGER PRIMARY KEY CHECK (id = 1), -- 单例主键；固定为 1。
    engine              TEXT NOT NULL DEFAULT 'system' CHECK (engine IN ('system', 'tesseract', 'paddleocr', 'cloud')), -- OCR 引擎：system=系统内置，tesseract/paddleocr=本地，cloud=云端。
    api_base            TEXT, -- 云端 OCR API 基础地址；本地引擎为空。
    secret_ref          TEXT, -- 云端 OCR 凭据的系统钥匙串引用；密钥本体禁止写入 SQLite。
    recognition_languages_json TEXT NOT NULL DEFAULT '["en"]' CHECK (json_valid(recognition_languages_json) AND json_type(recognition_languages_json) = 'array'), -- JSON 字符串数组；BCP 47 语言列表，例如 ["en","ja"]。
    sample_fps          REAL NOT NULL DEFAULT 2.0 CHECK (sample_fps > 0 AND sample_fps <= 30), -- 屏幕采样帧率；大于 0 且不超过 30 fps。
    min_confidence      REAL NOT NULL DEFAULT 0.75 CHECK (min_confidence BETWEEN 0 AND 1), -- OCR 最低置信度；0～1。
    deduplicate_frames  INTEGER NOT NULL DEFAULT 1 CHECK (deduplicate_frames IN (0, 1)), -- 连续帧去重：0=关闭，1=开启。
    options_json        TEXT NOT NULL DEFAULT '{}' CHECK (json_valid(options_json) AND json_type(options_json) = 'object'), -- JSON 对象；引擎专属参数，例如 {"use_gpu":false,"device":"cpu"}。
    updated_at          INTEGER NOT NULL DEFAULT (CAST(strftime('%s', 'now') AS INTEGER) * 1000) -- 最后更新时间；Unix 毫秒时间戳。
) STRICT;

INSERT INTO sys_ocr_settings(id) VALUES (1);

CREATE TABLE sys_shortcut_bindings ( -- 表：保存应用动作与键盘或手柄输入之间的绑定。
    action          TEXT NOT NULL, -- 动作：translate_once=单次翻译，auto_start=开始自动翻译，auto_stop=停止自动翻译；允许未来扩展。
    input_kind      TEXT NOT NULL CHECK (input_kind IN ('keyboard', 'gamepad')), -- 输入类型：keyboard=键盘，gamepad=手柄。
    binding_key     TEXT, -- 用于冲突检测的规范化输入，例如 CommandOrControl+Shift+KeyT；NULL=未绑定。
    binding_json    TEXT CHECK (binding_json IS NULL OR (json_valid(binding_json) AND json_type(binding_json) = 'object')), -- JSON 对象；键盘 {"modifiers":["CommandOrControl","Shift"],"code":"KeyT"}，手柄 {"gamepad_id":"standard","button":0}；NULL=未绑定。
    is_enabled      INTEGER NOT NULL DEFAULT 1 CHECK (is_enabled IN (0, 1)), -- 生效状态：0=停用此绑定，1=启用。
    updated_at      INTEGER NOT NULL DEFAULT (CAST(strftime('%s', 'now') AS INTEGER) * 1000), -- 最后更新时间；Unix 毫秒时间戳。
    PRIMARY KEY (action, input_kind), -- 同一动作在每种输入类型下最多保存一条绑定。
    CHECK ( -- 未绑定时 binding_key/binding_json 必须同时为空；已绑定时必须同时有值。
        (binding_key IS NULL AND binding_json IS NULL) OR
        (binding_key IS NOT NULL AND binding_json IS NOT NULL)
    )
) STRICT, WITHOUT ROWID;

-- 唯一索引：同一输入类型下，一个已启用的规范化输入只能绑定一个动作。
CREATE UNIQUE INDEX ux_sys_shortcut_bindings_active_input
    ON sys_shortcut_bindings(input_kind, binding_key)
    WHERE is_enabled = 1 AND binding_key IS NOT NULL;

INSERT INTO sys_shortcut_bindings(action, input_kind, binding_key, binding_json) VALUES
    ('translate_once', 'keyboard', 'CommandOrControl+Shift+KeyT', '{"modifiers":["CommandOrControl","Shift"],"code":"KeyT"}'),
    ('auto_start',     'keyboard', 'CommandOrControl+Shift+KeyA', '{"modifiers":["CommandOrControl","Shift"],"code":"KeyA"}'),
    ('auto_stop',      'keyboard', 'CommandOrControl+Shift+KeyS', '{"modifiers":["CommandOrControl","Shift"],"code":"KeyS"}'),
    ('translate_once', 'gamepad', NULL, NULL),
    ('auto_start',     'gamepad', NULL, NULL),
    ('auto_stop',      'gamepad', NULL, NULL);

CREATE TABLE sys_window_states ( -- 表：保存各 Tauri 原生窗口的位置、尺寸和置顶状态。
    window_label    TEXT PRIMARY KEY, -- 稳定窗口标签，例如 main、overlay；不得使用运行时 HWND。
    x               INTEGER, -- 窗口左上角 X 屏幕坐标；NULL=由系统决定。
    y               INTEGER, -- 窗口左上角 Y 屏幕坐标；NULL=由系统决定。
    width           INTEGER CHECK (width IS NULL OR width > 0), -- 窗口宽度（逻辑像素）；NULL=使用应用默认值。
    height          INTEGER CHECK (height IS NULL OR height > 0), -- 窗口高度（逻辑像素）；NULL=使用应用默认值。
    always_on_top   INTEGER NOT NULL DEFAULT 0 CHECK (always_on_top IN (0, 1)), -- 置顶状态：0=普通层级，1=始终置顶。
    updated_at      INTEGER NOT NULL DEFAULT (CAST(strftime('%s', 'now') AS INTEGER) * 1000), -- 最后更新时间；Unix 毫秒时间戳。
    CHECK ((x IS NULL AND y IS NULL) OR (x IS NOT NULL AND y IS NOT NULL)) -- 窗口位置必须同时保存 X/Y，或同时交由系统决定。
) STRICT, WITHOUT ROWID;

CREATE TABLE sys_capture_settings ( -- 表：保存全局唯一的屏幕捕获模式、区域及目标应用选择器。
    id                  INTEGER PRIMARY KEY CHECK (id = 1), -- 单例主键；固定为 1。
    mode                TEXT NOT NULL DEFAULT 'app' CHECK (mode IN ('app', 'region')), -- 捕获模式：app=目标应用窗口，region=自定义屏幕区域。
    region_x            INTEGER, -- 区域左上角 X 屏幕坐标；app 模式或未选择时为空。
    region_y            INTEGER, -- 区域左上角 Y 屏幕坐标；app 模式或未选择时为空。
    region_width        INTEGER CHECK (region_width IS NULL OR region_width > 0), -- 区域宽度（逻辑像素）；未选择时为空。
    region_height       INTEGER CHECK (region_height IS NULL OR region_height > 0), -- 区域高度（逻辑像素）；未选择时为空。
    app_selector_json   TEXT CHECK (app_selector_json IS NULL OR (json_valid(app_selector_json) AND json_type(app_selector_json) = 'object')), -- JSON 对象：{"platform":"windows","executable":"vlc.exe","bundle_id":null,"title_hint":"VLC"}；禁止保存 HWND。
    updated_at          INTEGER NOT NULL DEFAULT (CAST(strftime('%s', 'now') AS INTEGER) * 1000), -- 最后更新时间；Unix 毫秒时间戳。
    CHECK ( -- 捕获区域必须完整保存四个值，或四个值全部为空。
        (region_x IS NULL AND region_y IS NULL AND region_width IS NULL AND region_height IS NULL) OR
        (region_x IS NOT NULL AND region_y IS NOT NULL AND region_width IS NOT NULL AND region_height IS NOT NULL)
    )
) STRICT;

INSERT INTO sys_capture_settings(id) VALUES (1);

-- =============================================================================
-- SUB: subtitle library and search index
-- =============================================================================
CREATE TABLE sub_files ( -- 表：保存已导入字幕文件的元数据和索引状态，不保存文件二进制内容。
    id              INTEGER PRIMARY KEY, -- 字幕文件本地整数主键。
    path            TEXT NOT NULL UNIQUE, -- 规范化绝对路径；由应用处理平台路径大小写规则。
    file_name       TEXT NOT NULL, -- 用于界面显示的文件名，包含扩展名。
    format          TEXT NOT NULL CHECK (format IN ('srt', 'ass', 'vtt')), -- 字幕格式：srt=SubRip，ass=Advanced SubStation Alpha，vtt=WebVTT。
    fingerprint     TEXT NOT NULL UNIQUE CHECK (length(fingerprint) = 64 AND fingerprint NOT GLOB '*[^0-9a-f]*'), -- 文件内容 SHA-256 小写十六进制指纹，用于阻止重复导入。
    source_language TEXT, -- 字幕原文语言 BCP 47 标识；未知时为空。
    target_language TEXT, -- 字幕译文语言 BCP 47 标识；单语字幕时为空。
    file_size_bytes INTEGER NOT NULL CHECK (file_size_bytes >= 0), -- 文件大小，单位字节。
    file_modified_at INTEGER, -- 源文件最后修改时间；Unix 毫秒时间戳。
    index_status    TEXT NOT NULL DEFAULT 'pending' CHECK (index_status IN ('pending', 'indexing', 'ready', 'failed')), -- 索引状态：pending=待处理，indexing=处理中，ready=可检索，failed=失败。
    entry_count     INTEGER NOT NULL DEFAULT 0 CHECK (entry_count >= 0), -- 已解析字幕条目数量。
    error_message   TEXT, -- 索引失败详情；仅 index_status=failed 时使用。
    created_at      INTEGER NOT NULL DEFAULT (CAST(strftime('%s', 'now') AS INTEGER) * 1000), -- 导入时间；Unix 毫秒时间戳。
    updated_at      INTEGER NOT NULL DEFAULT (CAST(strftime('%s', 'now') AS INTEGER) * 1000) -- 最后更新时间；Unix 毫秒时间戳。
) STRICT;

-- 普通索引：按索引状态筛选字幕文件，例如查找 pending/failed 任务。
CREATE INDEX ix_sub_files_status ON sub_files(index_status);

CREATE TABLE sub_entries ( -- 表：保存字幕文件解析后的时间轴条目和双语文本。
    id              INTEGER PRIMARY KEY, -- 字幕条目本地整数主键。
    subtitle_file_id INTEGER NOT NULL REFERENCES sub_files(id) ON DELETE CASCADE, -- 所属字幕文件；删除文件时级联删除条目。
    sequence_no     INTEGER NOT NULL CHECK (sequence_no >= 0), -- 条目在原字幕文件中的顺序号，从 0 或 1 开始均可但同一文件须一致。
    start_ms        INTEGER CHECK (start_ms IS NULL OR start_ms >= 0), -- 字幕开始时间，单位毫秒；无时间轴时为空。
    end_ms          INTEGER CHECK (end_ms IS NULL OR end_ms >= start_ms), -- 字幕结束时间，单位毫秒；不得早于 start_ms。
    source_text     TEXT NOT NULL CHECK (length(trim(source_text)) > 0), -- 字幕原文；保留用于展示的非空正文文本。
    translated_text TEXT, -- 字幕译文；单语字幕或尚未配对时为空。
    UNIQUE (subtitle_file_id, sequence_no), -- 同一字幕文件内的顺序号不可重复。
    CHECK ( -- 时间轴必须同时有开始/结束时间，或两者同时为空。
        (start_ms IS NULL AND end_ms IS NULL) OR
        (start_ms IS NOT NULL AND end_ms IS NOT NULL AND end_ms >= start_ms)
    )
) STRICT;

-- 普通索引：按字幕文件和开始时间顺序读取时间轴。
CREATE INDEX ix_sub_entries_file_timeline
    ON sub_entries(subtitle_file_id, start_ms);

CREATE VIRTUAL TABLE sub_entries_fts USING fts5( -- 虚表：sub_entries 原文和译文的 FTS5 trigram 外部内容索引。
    source_text, -- 索引 sub_entries.source_text。
    translated_text, -- 索引 sub_entries.translated_text；NULL 按空字符串写入。
    content = 'sub_entries', -- 外部内容表；正文只保存在 sub_entries。
    content_rowid = 'id', -- FTS rowid 对应 sub_entries.id。
    tokenize = 'trigram' -- 三字符切分，支持中英文子串召回；不足 3 字符时应用层回退或忽略。
);

-- 触发器：新增字幕条目后同步写入 FTS 索引。
CREATE TRIGGER trg_sub_entries_fts_ai AFTER INSERT ON sub_entries BEGIN
    INSERT INTO sub_entries_fts(rowid, source_text, translated_text)
    VALUES (new.id, new.source_text, coalesce(new.translated_text, ''));
END;

-- 触发器：删除字幕条目后同步删除 FTS 索引内容。
CREATE TRIGGER trg_sub_entries_fts_ad AFTER DELETE ON sub_entries BEGIN
    INSERT INTO sub_entries_fts(sub_entries_fts, rowid, source_text, translated_text)
    VALUES ('delete', old.id, old.source_text, coalesce(old.translated_text, ''));
END;

-- 触发器：字幕原文或译文改变后重建对应 FTS 索引内容。
CREATE TRIGGER trg_sub_entries_fts_au AFTER UPDATE OF source_text, translated_text ON sub_entries BEGIN
    INSERT INTO sub_entries_fts(sub_entries_fts, rowid, source_text, translated_text)
    VALUES ('delete', old.id, old.source_text, coalesce(old.translated_text, ''));
    INSERT INTO sub_entries_fts(rowid, source_text, translated_text)
    VALUES (new.id, new.source_text, coalesce(new.translated_text, ''));
END;

-- =============================================================================
-- TR: translation sessions, history, search index, and cache
-- =============================================================================
CREATE TABLE tr_sessions ( -- 表：保存一次手动或自动翻译运行及其开始时的上下文快照。
    id                  INTEGER PRIMARY KEY, -- 翻译会话本地整数主键。
    mode                TEXT NOT NULL CHECK (mode IN ('manual', 'auto')), -- 会话模式：manual=单次翻译，auto=持续翻译。
    capture_context_json TEXT CHECK (capture_context_json IS NULL OR (json_valid(capture_context_json) AND json_type(capture_context_json) = 'object')), -- JSON 对象：{"mode":"region","region":{"x":700,"y":46,"width":520,"height":118},"app":null}。
    settings_snapshot_json TEXT NOT NULL CHECK (json_valid(settings_snapshot_json) AND json_type(settings_snapshot_json) = 'object'), -- JSON 对象：{"model_profile_id":1,"translation":{...},"ocr":{...}}；用于保留会话开始时配置。
    started_at          INTEGER NOT NULL DEFAULT (CAST(strftime('%s', 'now') AS INTEGER) * 1000), -- 会话开始时间；Unix 毫秒时间戳。
    ended_at            INTEGER CHECK (ended_at IS NULL OR ended_at >= started_at) -- 会话结束时间；进行中为空，不得早于 started_at。
) STRICT;

CREATE TABLE tr_history ( -- 表：保存每次翻译尝试的原文、结果、来源、性能和错误信息。
    id                  INTEGER PRIMARY KEY, -- 翻译历史本地整数主键。
    session_id          INTEGER REFERENCES tr_sessions(id) ON DELETE SET NULL, -- 所属翻译会话；会话删除后保留历史并置空。
    model_profile_id    INTEGER REFERENCES sys_model_profiles(id) ON DELETE SET NULL, -- 使用的模型配置；配置删除后置空。
    subtitle_file_id    INTEGER REFERENCES sub_files(id) ON DELETE SET NULL, -- 命中的字幕文件；非字幕结果或文件删除后为空。
    source_language     TEXT NOT NULL, -- 实际源语言 BCP 47 标识；自动检测后应写识别结果而非 auto。
    target_language     TEXT NOT NULL, -- 目标语言 BCP 47 标识。
    source_text         TEXT NOT NULL CHECK (length(trim(source_text)) > 0), -- 本次提交翻译的非空规范化原文。
    translated_text     TEXT, -- 翻译结果；失败或取消时可为空。
    source_hash         TEXT NOT NULL CHECK (length(source_hash) = 64 AND source_hash NOT GLOB '*[^0-9a-f]*'), -- 规范化原文的 SHA-256 小写十六进制摘要。
    result_source       TEXT NOT NULL CHECK (result_source IN ('model', 'subtitle', 'cache')), -- 结果来源：model=模型请求，subtitle=字幕命中，cache=翻译缓存命中。
    provider_snapshot   TEXT, -- 当时服务商标识快照；模型配置删除或修改后仍可解释历史。
    model_snapshot      TEXT, -- 当时模型名称快照；字幕结果可为空。
    status              TEXT NOT NULL DEFAULT 'success' CHECK (status IN ('success', 'error', 'cancelled')), -- 执行状态：success=成功，error=失败，cancelled=主动取消。
    latency_ms          INTEGER CHECK (latency_ms IS NULL OR latency_ms >= 0), -- 处理耗时，单位毫秒；未知时为空。
    input_tokens        INTEGER CHECK (input_tokens IS NULL OR input_tokens >= 0), -- 模型输入 token 数；非模型结果或服务商未返回时为空。
    output_tokens       INTEGER CHECK (output_tokens IS NULL OR output_tokens >= 0), -- 模型输出 token 数；非模型结果或服务商未返回时为空。
    error_code          TEXT, -- 稳定错误码；仅 status=error 时使用。
    error_message       TEXT, -- 可读错误详情；不得包含 API Key 等敏感信息。
    is_favorite         INTEGER NOT NULL DEFAULT 0 CHECK (is_favorite IN (0, 1)), -- 收藏状态：0=未收藏，1=收藏；自动清理不得删除收藏项。
    created_at          INTEGER NOT NULL DEFAULT (CAST(strftime('%s', 'now') AS INTEGER) * 1000), -- 历史创建时间；Unix 毫秒时间戳。
    updated_at          INTEGER NOT NULL DEFAULT (CAST(strftime('%s', 'now') AS INTEGER) * 1000), -- 最后更新时间；收藏状态等可变字段更新时写入。
    CHECK (status <> 'success' OR translated_text IS NOT NULL) -- 成功记录必须包含译文。
) STRICT;

-- 普通索引：支持历史列表按创建时间倒序稳定分页。
CREATE INDEX ix_tr_history_created
    ON tr_history(created_at DESC, id DESC);
-- 普通索引：支持按源语言和目标语言筛选历史。
CREATE INDEX ix_tr_history_languages
    ON tr_history(source_language, target_language, created_at DESC);
-- 普通索引：支持按模型配置筛选历史。
CREATE INDEX ix_tr_history_model
    ON tr_history(model_profile_id, created_at DESC);
-- 部分索引：仅索引收藏记录，支持收藏列表按时间倒序读取。
CREATE INDEX ix_tr_history_favorites
    ON tr_history(created_at DESC)
    WHERE is_favorite = 1;

CREATE VIRTUAL TABLE tr_history_fts USING fts5( -- 虚表：tr_history 原文和译文的 FTS5 trigram 外部内容索引。
    source_text, -- 索引 tr_history.source_text。
    translated_text, -- 索引 tr_history.translated_text；NULL 按空字符串写入。
    content = 'tr_history', -- 外部内容表；正文只保存在 tr_history。
    content_rowid = 'id', -- FTS rowid 对应 tr_history.id。
    tokenize = 'trigram' -- 三字符切分，支持中英文子串检索；不足 3 字符时应用层使用受限 LIKE。
);

-- 触发器：新增历史后同步写入 FTS 索引。
CREATE TRIGGER trg_tr_history_fts_ai AFTER INSERT ON tr_history BEGIN
    INSERT INTO tr_history_fts(rowid, source_text, translated_text)
    VALUES (new.id, new.source_text, coalesce(new.translated_text, ''));
END;

-- 触发器：删除历史后同步删除 FTS 索引内容。
CREATE TRIGGER trg_tr_history_fts_ad AFTER DELETE ON tr_history BEGIN
    INSERT INTO tr_history_fts(tr_history_fts, rowid, source_text, translated_text)
    VALUES ('delete', old.id, old.source_text, coalesce(old.translated_text, ''));
END;

-- 触发器：历史原文或译文改变后重建对应 FTS 索引内容。
CREATE TRIGGER trg_tr_history_fts_au AFTER UPDATE OF source_text, translated_text ON tr_history BEGIN
    INSERT INTO tr_history_fts(tr_history_fts, rowid, source_text, translated_text)
    VALUES ('delete', old.id, old.source_text, coalesce(old.translated_text, ''));
    INSERT INTO tr_history_fts(rowid, source_text, translated_text)
    VALUES (new.id, new.source_text, coalesce(new.translated_text, ''));
END;

-- 缓存与历史相互独立：删除历史不得改变缓存行为。
CREATE TABLE tr_cache ( -- 表：保存可淘汰的模型翻译缓存；与用户可见的翻译历史相互独立。
    cache_key           TEXT PRIMARY KEY CHECK (length(cache_key) = 64 AND cache_key NOT GLOB '*[^0-9a-f]*'), -- source_hash、语言、模型和 config_hash 组合后的 SHA-256 缓存键。
    source_hash         TEXT NOT NULL CHECK (length(source_hash) = 64 AND source_hash NOT GLOB '*[^0-9a-f]*'), -- 规范化原文的 SHA-256 小写十六进制摘要。
    source_text         TEXT NOT NULL CHECK (length(trim(source_text)) > 0), -- 原文副本，用于校验极低概率哈希碰撞和诊断。
    source_language     TEXT NOT NULL, -- 源语言 BCP 47 标识。
    target_language     TEXT NOT NULL, -- 目标语言 BCP 47 标识。
    model_profile_id    INTEGER NOT NULL REFERENCES sys_model_profiles(id) ON DELETE CASCADE, -- 产生缓存的模型配置；删除配置时级联清理缓存。
    config_hash         TEXT NOT NULL CHECK (length(config_hash) = 64 AND config_hash NOT GLOB '*[^0-9a-f]*'), -- 影响译文的参数、提示词和模型版本所组成配置的 SHA-256 摘要。
    translated_text     TEXT NOT NULL, -- 缓存的翻译结果。
    hit_count           INTEGER NOT NULL DEFAULT 0 CHECK (hit_count >= 0), -- 累计命中次数。
    created_at          INTEGER NOT NULL DEFAULT (CAST(strftime('%s', 'now') AS INTEGER) * 1000), -- 缓存创建时间；Unix 毫秒时间戳。
    last_accessed_at    INTEGER NOT NULL DEFAULT (CAST(strftime('%s', 'now') AS INTEGER) * 1000), -- 最近命中时间；Unix 毫秒时间戳，用于 LRU 清理。
    expires_at          INTEGER, -- 过期时间；Unix 毫秒时间戳，NULL=不过期。
    CHECK (last_accessed_at >= created_at), -- 最近访问时间不得早于缓存创建时间。
    CHECK (expires_at IS NULL OR expires_at > created_at) -- 到期时间必须晚于缓存创建时间。
) STRICT, WITHOUT ROWID;

-- 普通索引：按过期时间批量清理缓存。
CREATE INDEX ix_tr_cache_expiry ON tr_cache(expires_at);
-- 普通索引：按最近访问时间执行 LRU 容量清理。
CREATE INDEX ix_tr_cache_lru ON tr_cache(last_accessed_at);

-- 记录首版 migration 已完成；db_schema_migrations 是唯一迁移版本来源。
INSERT INTO db_schema_migrations(version, name) VALUES (1, 'initial');

COMMIT;
