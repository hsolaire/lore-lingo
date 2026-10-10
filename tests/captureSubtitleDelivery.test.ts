import { afterEach, describe, expect, it, vi } from 'vitest'
import { ref } from 'vue'
import type { CaptureTranslationEvent } from '@/types/capturedLayout'

let deliver!: (event: CaptureTranslationEvent) => void
const srcText = ref('')
const tgtText = ref('')

vi.mock('@/services/captureTranslationClient', () => ({ captureTranslationClient: {
  onState: vi.fn((handler: (event: CaptureTranslationEvent) => void) => {
    deliver = handler
    return Promise.resolve(vi.fn())
  }),
  start: vi.fn(() => Promise.resolve({ sessionId: 'current' })),
  getConfig: vi.fn(() => Promise.resolve({ endpoint: 'http://127.0.0.1:11434/v1', model: 'test-model', apiKey: '', sourceLanguage: 'auto', targetLanguage: 'zh-CN' })),
} }))
vi.mock('@/composables/useTranslator', () => ({ useTranslator: () => ({ srcText, tgtText, langA: ref('en'), langB: ref('zh-CN') }) }))

import { useCaptureTranslation } from '@/composables/useCaptureTranslation'

afterEach(() => { delete (window as Window & { __TAURI_INTERNALS__?: object }).__TAURI_INTERNALS__ })

describe('截图结果交付到合并字幕', () => {
  it('按阅读顺序显示真实原文和译文，忽略过期会话', async () => {
    Object.defineProperty(window, '__TAURI_INTERNALS__', { value: {}, configurable: true })
    await useCaptureTranslation().start()
    const event: CaptureTranslationEvent = {
      sessionId: 'current', state: 'success', error: null,
      document: {
        imageWidth: 100, imageHeight: 100, imagePngB64: '', sourceLanguage: 'en', targetLanguage: 'zh-CN',
        blocks: [
          { id: 'second', kind: 'paragraph', bounds: { x: 0, y: 0.5, width: 1, height: 0.5 }, sourceText: 'Second', translatedText: '第二行', readingOrder: 1, confidence: 1 },
          { id: 'first', kind: 'title', bounds: { x: 0, y: 0, width: 1, height: 0.5 }, sourceText: 'First', translatedText: '第一行', readingOrder: 0, confidence: 1 },
        ],
      },
    }
    deliver(event)
    expect(srcText.value).toBe('First\nSecond')
    expect(tgtText.value).toBe('第一行\n第二行')
    deliver({ ...event, sessionId: 'expired', document: { ...event.document!, blocks: [] } })
    expect(tgtText.value).toBe('第一行\n第二行')
    deliver({ ...event, document: { ...event.document!, blocks: event.document!.blocks.map(block => ({ ...block, translatedText: null })) } })
    expect(srcText.value).toBe('First\nSecond')
    expect(tgtText.value).toBe('')
  })
})
