export type LiveSessionStatus = 'idle' | 'starting' | 'running' | 'paused' | 'stopping' | 'error'
export type LayoutTaskStatus =
  | 'idle'
  | 'selecting'
  | 'capturing'
  | 'recognizing'
  | 'translating'
  | 'completed'
  | 'cancelled'
  | 'failed'

export interface ActiveSessionSummary {
  sessionId: number
  mode: string
  status: LiveSessionStatus
  startedAt: number
}

export interface LatestTranslationSummary {
  historyId: number
  sourceText: string
  translatedText: string | null
  sourceLanguage: string
  targetLanguage: string
  resultSource: string
  status: string
  createdAt: number
}

export interface PublicError {
  code: string
  message: string
  recoverable: boolean
  action?: string
}

const isRecord = (value: unknown): value is Record<string, unknown> =>
  typeof value === 'object' && value !== null

export function toPublicError(value: unknown): PublicError {
  if (
    isRecord(value)
    && typeof value.code === 'string'
    && typeof value.message === 'string'
    && typeof value.recoverable === 'boolean'
    && (value.action === undefined || typeof value.action === 'string')
  ) {
    return {
      code: value.code,
      message: value.message,
      recoverable: value.recoverable,
      ...(typeof value.action === 'string' ? { action: value.action } : {}),
    }
  }

  return {
    code: 'backend_error',
    message: '后台服务暂时不可用，请稍后重试。',
    recoverable: true,
    action: 'retry',
  }
}
