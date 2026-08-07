import type {
  CapturePermission,
  CaptureSourceCatalog,
  DesktopCapabilities,
} from '@/types/desktop'
import { invokeCommand } from './tauriClient'

export const desktopClient = {
  capabilities(): Promise<DesktopCapabilities> {
    return invokeCommand<DesktopCapabilities>('get_desktop_capabilities')
  },
  capturePermission(): Promise<CapturePermission> {
    return invokeCommand<CapturePermission>('capture_permission_status')
  },
  requestCapturePermission(): Promise<CapturePermission> {
    return invokeCommand<CapturePermission>('request_capture_permission')
  },
  sources(): Promise<CaptureSourceCatalog> {
    return invokeCommand<CaptureSourceCatalog>('list_capture_sources')
  },
}
