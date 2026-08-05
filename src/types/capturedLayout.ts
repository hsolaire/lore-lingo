export type CaptureTranslationState =
  | 'selecting'
  | 'processing'
  | 'empty'
  | 'success'
  | 'error'
  | 'cancelled'

export type LayoutBlockKind = 'title' | 'paragraph' | 'list-item'

export interface LogicalRect {
  x: number
  y: number
  width: number
  height: number
}

export interface NormalizedRect extends LogicalRect {}

export interface LayoutBlock {
  id: string
  kind: LayoutBlockKind
  bounds: NormalizedRect
  sourceText: string
  translatedText: string | null
  readingOrder: number
  confidence: number
}

export interface LayoutDocument {
  imageWidth: number
  imageHeight: number
  imagePngB64: string
  sourceLanguage: string
  targetLanguage: string
  blocks: LayoutBlock[]
}

export interface TranslationConfig {
  endpoint: string
  model: string
  apiKey: string
  sourceLanguage: string
  targetLanguage: string
}

export interface PublicError {
  code: string
  message: string
  recoverable: boolean
  action?: string
}

export interface CaptureTranslationEvent {
  sessionId: string
  state: CaptureTranslationState
  document: LayoutDocument | null
  error: PublicError | null
}

export interface SelectorPayload {
  sessionId: string
  monitorId: string
  imagePngB64: string
  logicalWidth: number
  logicalHeight: number
}
