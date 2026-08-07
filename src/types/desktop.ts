export type PlatformKind = 'macos' | 'windows' | 'unsupported'
export type CaptureSourceKind = 'screen' | 'window'
export type CapturePermissionStatus =
  | 'not_required'
  | 'not_determined'
  | 'granted'
  | 'denied'
  | 'restart_required'
  | 'unsupported'
export type RecoveryAction = 'open_system_settings' | 'restart_application' | 'retry'

export interface PlatformCapabilities {
  platform: PlatformKind
  screenCapture: boolean
  windowCapture: boolean
  systemOcr: boolean
  globalShortcuts: boolean
  credentialStorage: boolean
}

export interface DesktopCapabilities extends PlatformCapabilities {
  architecture: string
  regionCapture: boolean
  permissionPrompt: boolean
  permissionRestartRequired: boolean
  selfExclusion: boolean
}

export interface CapturePermission {
  status: CapturePermissionStatus
  canRequest: boolean
  action: RecoveryAction | null
}

export interface LogicalSize {
  width: number
  height: number
}

export interface PhysicalSize {
  width: number
  height: number
}

export interface PhysicalPoint {
  x: number
  y: number
}

export interface MonitorSummary {
  monitorId: string
  primary: boolean
  logicalSize: LogicalSize
  physicalSize: PhysicalSize
  scaleFactor: number
  physicalOrigin: PhysicalPoint
}

export interface CaptureSourceSummary {
  sourceId: string
  catalogRevision: number
  kind: CaptureSourceKind
  title: string
  applicationName: string | null
  iconDataUrl: string | null
  monitor: MonitorSummary | null
  capturable: boolean
}

export interface CaptureSourceCatalog {
  catalogRevision: number
  sources: CaptureSourceSummary[]
}

export interface DesktopError {
  code: string
  message: string
  recoverable: boolean
  action: RecoveryAction | null
}

export interface WindowSummary {
  label: string
  visible: boolean
  alwaysOnTop: boolean
}
