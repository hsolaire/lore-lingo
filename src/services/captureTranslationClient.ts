import type {
  CaptureTranslationEvent,
  LogicalRect,
  SelectorPayload,
  TranslationConfig,
  CapturedRegion, RegionBounds, RegionPreview,
} from '@/types/capturedLayout'
import { invokeCommand, listenEvent } from './tauriClient'

const stateEvent = 'capture-translation:state'

export const captureTranslationClient = {
  region(): Promise<CapturedRegion | null> { return invokeCommand('get_captured_region') },
  updateRegion(revision: number, bounds: RegionBounds): Promise<CapturedRegion> { return invokeCommand('update_captured_region', { request: { revision, bounds } }) },
  regionPreview(revision: number): Promise<RegionPreview> { return invokeCommand('captured_region_preview', { revision }) },
  translateRegion(revision: number, translation: TranslationConfig): Promise<{ sessionId: string }> { return invokeCommand('translate_captured_region', { revision, translation }) },
  getConfig(): Promise<TranslationConfig> { return invokeCommand('get_capture_translation_config') },
  saveConfig(config: TranslationConfig): Promise<TranslationConfig> { return invokeCommand('save_capture_translation_config', { config }) },
  startWindow(sourceId: string, catalogRevision: number, translation: TranslationConfig): Promise<{ sessionId: string }> {
    return invokeCommand('capture_window_translation', { request: { sourceId, catalogRevision, translation } })
  },
  start(translation: TranslationConfig): Promise<{ sessionId: string }> {
    return invokeCommand('start_capture_translation', { request: { translation } })
  },
  selectorPayload(): Promise<SelectorPayload> {
    return invokeCommand('capture_selector_payload')
  },
  complete(sessionId: string, monitorId: string, rect: LogicalRect): Promise<void> {
    return invokeCommand('complete_capture_translation', {
      request: { sessionId, monitorId, rect },
    })
  },
  cancel(sessionId: string): Promise<void> {
    return invokeCommand('cancel_capture_translation', { request: { sessionId } })
  },
  onState(handler: (event: CaptureTranslationEvent) => void) {
    return listenEvent<CaptureTranslationEvent>(stateEvent, event => handler(event.payload))
  },
}
