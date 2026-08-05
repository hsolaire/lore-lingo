export type TranslationMode = 'manual' | 'auto'
export type CaptureMode = 'region' | 'window' | 'app' | 'display'

export interface ModelProfileSummary {
  id: number
  name: string
  provider: string
  modelName: string
  apiBase: string | null
  hasSecret: boolean
  options: Record<string, unknown>
  isEnabled: boolean
}

export interface TranslationSettingsSummary {
  defaultModelProfileId: number | null
  mode: TranslationMode
  autoIntervalMs: number
  sourceLanguage: string
  targetLanguage: string
  requestTimeoutMs: number
  preferSubtitles: boolean
  saveHistory: boolean
  historyRetentionDays: number | null
}

export interface OcrSettingsSummary {
  engine: string
  apiBase: string | null
  hasSecret: boolean
  recognitionLanguages: string[]
  sampleFps: number
  minConfidence: number
  deduplicateFrames: boolean
  options: Record<string, unknown>
}

export interface ShortcutBindingSummary {
  action: string
  inputKind: string
  bindingKey: string | null
  binding: Record<string, unknown> | null
  isEnabled: boolean
}

export interface CaptureRegionSummary {
  x: number
  y: number
  width: number
  height: number
}

export interface CaptureSettingsSummary {
  mode: CaptureMode
  region: CaptureRegionSummary | null
  appSelector: Record<string, unknown> | null
}

export interface SettingsSummary {
  translation: TranslationSettingsSummary
  ocr: OcrSettingsSummary
}
