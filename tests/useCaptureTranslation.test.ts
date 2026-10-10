import { describe, expect, it } from 'vitest'

import type { CaptureTranslationEvent, CaptureTranslationViewState, LayoutDocument } from '@/types/capturedLayout'
import { reduceCaptureTranslationState } from '@/composables/useCaptureTranslation'

const idle: CaptureTranslationViewState = { status: 'idle', sessionId: null, document: null, error: null }
const document: LayoutDocument = {
  imageWidth: 100,
  imageHeight: 60,
  imagePngB64: 'cG5n',
  sourceLanguage: 'en',
  targetLanguage: 'zh-CN',
  blocks: [],
}

function event(overrides: Partial<CaptureTranslationEvent>): CaptureTranslationEvent {
  return { sessionId: 'capture-1', state: 'selecting', document: null, error: null, ...overrides }
}

describe('截图翻译状态归约', () => {
  it('从选择、处理进入空结果和成功结果', () => {
    const selecting = reduceCaptureTranslationState(idle, event({}))
    const processing = reduceCaptureTranslationState(selecting, event({ state: 'processing' }))
    expect(selecting.status).toBe('selecting')
    expect(processing.status).toBe('processing')
    expect(reduceCaptureTranslationState(processing, event({ state: 'empty', document })).status).toBe('empty')
    expect(reduceCaptureTranslationState(processing, event({ state: 'success', document })).status).toBe('success')
  })

  it('捕获后在处理中和识别失败时仍保留真实预览', () => {
    const processing = reduceCaptureTranslationState(idle, event({ state: 'processing', document }))
    expect(processing.document).toBe(document)
    const failed = reduceCaptureTranslationState(processing, event({ state: 'error', error: { code: 'ocr_failure', message: '识别失败', recoverable: true } }))
    expect(failed.document).toBe(document)
    expect(failed.status).toBe('error')
    expect(reduceCaptureTranslationState(failed, event({ state: 'cancelled' })).document).toBeNull()
  })

  it('忽略其他会话的过期事件', () => {
    const current: CaptureTranslationViewState = {
      status: 'processing', sessionId: 'capture-current', document: null, error: null,
    }
    expect(reduceCaptureTranslationState(current, event({ sessionId: 'capture-stale', state: 'error' }))).toBe(current)
  })

  it('保留结构化错误并进入取消终态', () => {
    const failed = reduceCaptureTranslationState(idle, event({
      state: 'error',
      error: { code: 'ocr_failure', message: '识别失败', recoverable: true },
    }))
    expect(failed).toMatchObject({ status: 'error', error: { code: 'ocr_failure' } })
    expect(reduceCaptureTranslationState(idle, event({ state: 'cancelled' })).status).toBe('cancelled')
  })
})
