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

export interface RegionBounds { x1: number; x2: number; y1: number; y2: number }
export interface CapturedRegion {
  revision: number
  monitorId: string
  logicalWidth: number
  logicalHeight: number
  bounds: RegionBounds
}
export interface RegionPreview {
  revision: number
  imagePngB64: string
  imageWidth: number
  imageHeight: number
  capturedAtMs: number
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

export type CaptureTranslationViewState =
  | { status: 'idle'; sessionId: null; document: null; error: null }
  | { status: 'selecting'; sessionId: string; document: null; error: null }
  | { status: 'processing'; sessionId: string; document: LayoutDocument | null; error: null }
  | { status: 'empty'; sessionId: string; document: LayoutDocument; error: null }
  | { status: 'success'; sessionId: string; document: LayoutDocument; error: null }
  | { status: 'error'; sessionId: string | null; document: LayoutDocument | null; error: PublicError }
  | { status: 'cancelled'; sessionId: string; document: null; error: null }
