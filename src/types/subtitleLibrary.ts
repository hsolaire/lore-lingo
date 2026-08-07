export type SubtitleIndexStatus = 'pending' | 'indexing' | 'ready' | 'error'

export interface SubtitleLibrarySummary {
  fileCount: number
  readyCount: number
  errorCount: number
}
