import type { PlatformCapabilities, WindowSummary } from './desktop'
import type { HistorySummary } from './history'
import type {
  CaptureSettingsSummary,
  ModelProfileSummary,
  SettingsSummary,
  ShortcutBindingSummary,
} from './settings'
import type { ActiveSessionSummary, LatestTranslationSummary } from './translation'

export interface BootstrapSnapshot {
  platform: PlatformCapabilities
  windows: WindowSummary[]
  settings: SettingsSummary
  defaultModel: ModelProfileSummary | null
  shortcuts: ShortcutBindingSummary[]
  capture: CaptureSettingsSummary
  activeSession: ActiveSessionSummary | null
  latestResult: LatestTranslationSummary | null
  history: HistorySummary
  settingsRevision: number
  captureRevision: number
  sessionRevision: number
}

export type BootstrapState =
  | { status: 'loading'; snapshot: null; error: null }
  | { status: 'ready'; snapshot: BootstrapSnapshot; error: null }
  | { status: 'database_error'; snapshot: null; error: string }
  | { status: 'unsupported'; snapshot: BootstrapSnapshot | null; error: string }
