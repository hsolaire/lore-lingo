import type { CaptureSettingsSummary } from '@/types/settings'
import { invokeCommand } from './tauriClient'

export const captureClient = {
  getSettings(): Promise<CaptureSettingsSummary> {
    return invokeCommand<CaptureSettingsSummary>('get_capture_settings')
  },
}
