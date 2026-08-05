export type PlatformKind = 'macos' | 'windows' | 'unsupported'
export type CaptureSourceKind = 'display' | 'window' | 'application'

export interface PlatformCapabilities {
  platform: PlatformKind
  screenCapture: boolean
  windowCapture: boolean
  systemOcr: boolean
  globalShortcuts: boolean
  credentialStorage: boolean
}

export interface CaptureSourceSummary {
  sourceId: string
  catalogRevision: number
  kind: CaptureSourceKind
  title: string
  applicationName: string | null
  iconDataUrl: string | null
}

export interface CaptureSourceCatalog {
  revision: number
  sources: CaptureSourceSummary[]
}

export interface WindowSummary {
  label: string
  visible: boolean
  alwaysOnTop: boolean
}
