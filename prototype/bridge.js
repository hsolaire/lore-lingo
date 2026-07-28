/* ============================================================
   LensLingo · 跨窗口状态桥 (bridge.js)
   三个独立窗口（主面板 / 字幕 / 设置）通过共享状态联动。

   Web/预览环境：localStorage + `storage` 事件在标签页/窗口间广播。
   真实桌面客户端：把 read/write/subscribe 换成 IPC / 共享 store 即可，
   页面侧调用不变（LensLingo.get / set / subscribe / openWindow）。
   ============================================================ */
(function (global) {
  const KEY = 'lenslingo:state';

  // 默认共享状态 —— 三个窗口都读写它
  const DEFAULT = {
    src:  "I never thought I'd find you here, of all places.",
    tgt:  "没想到会在这种地方遇见你。",
    from: "English",
    to:   "简体中文",
    hit:  "subtitle",          // subtitle=字幕库命中 / model=模型翻译
    capture: { mode: "app", name: "VLC · 媒体播放器", meta: "窗口 · 1920×1080 · 采样 2fps" },
    auto: true,
    pinned: false,
    settings: {
      provider: "OpenAI · GPT-4o mini",
      apiBase:  "https://api.openai.com/v1",
      ocrEngine: "macOS Vision（系统内置 · 离线）",
      fps: "2 fps · 均衡",
      translateMode: "auto",
      autoInterval: 1,
      hotkeyTranslate: "⌘⇧T",
      hotkeyAutoStart: "⌘⇧A",
      hotkeyAutoStop: "⌘⇧S",
      gamepadTranslate: "未绑定",
      gamepadAutoStart: "未绑定",
      gamepadAutoStop: "未绑定",
      subsCount: 2
    },
    rev: 0
  };

  function mergeState(saved) {
    return Object.assign({}, DEFAULT, saved, {
      settings: Object.assign({}, DEFAULT.settings, saved && saved.settings)
    });
  }

  function read() {
    try {
      const raw = localStorage.getItem(KEY);
      if (!raw) return mergeState();
      return mergeState(JSON.parse(raw));
    } catch (_) { return mergeState(); }
  }

  let state = read();
  const listeners = new Set();

  function emit(patchKeys) {
    listeners.forEach(fn => { try { fn(state, patchKeys); } catch (_) {} });
  }

  const API = {
    /** 读取整个状态（只读快照） */
    get(key) { return key ? state[key] : state; },

    /** 合并写入并广播；本窗口同步触发，其他窗口经 storage 事件收到 */
    set(patch) {
      state = Object.assign({}, state, patch, { rev: (state.rev || 0) + 1 });
      try { localStorage.setItem(KEY, JSON.stringify(state)); } catch (_) {}
      emit(Object.keys(patch));
    },

    /** 深层写 settings 子对象 */
    setSettings(patch) {
      API.set({ settings: Object.assign({}, state.settings, patch) });
    },

    /** 订阅变更，返回取消函数 */
    subscribe(fn) { listeners.add(fn); return () => listeners.delete(fn); },

    /**
     * 打开另一个窗口。
     * Web：window.open 新标签；真实客户端：主进程 createWindow。
     */
    openWindow(name) {
      const map = { main: 'index.html', subtitle: 'subtitle.html', settings: 'settings.html' };
      const file = map[name] || name;
      const feat = name === 'subtitle'
        ? 'width=820,height=260'
        : name === 'settings' ? 'width=520,height=680' : 'width=470,height=640';
      try { global.open(file, 'lenslingo-' + name, feat); } catch (_) { global.open(file, '_blank'); }
    }
  };

  // 跨窗口：其他窗口写入时刷新本地快照并广播
  global.addEventListener('storage', e => {
    if (e.key !== KEY || !e.newValue) return;
    try {
      const next = mergeState(JSON.parse(e.newValue));
      const changed = Object.keys(next).filter(k => JSON.stringify(next[k]) !== JSON.stringify(state[k]));
      state = next;
      emit(changed);
    } catch (_) {}
  });

  global.LensLingo = API;
})(window);
