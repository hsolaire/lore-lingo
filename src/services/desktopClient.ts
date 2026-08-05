import type { CaptureSourceCatalog, PlatformCapabilities } from '@/types/desktop'
import { invokeCommand } from './tauriClient'

export const desktopClient = {
  capabilities(): Promise<PlatformCapabilities> {
    return invokeCommand<PlatformCapabilities>('get_desktop_capabilities')
  },
  sources(): Promise<CaptureSourceCatalog> {
    return invokeCommand<CaptureSourceCatalog>('list_capture_sources')
  },
}
