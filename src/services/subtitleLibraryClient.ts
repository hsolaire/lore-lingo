import type { SubtitleLibrarySummary } from '@/types/subtitleLibrary'
import { invokeCommand } from './tauriClient'

export const subtitleLibraryClient = {
  summary(): Promise<SubtitleLibrarySummary> {
    return invokeCommand<SubtitleLibrarySummary>('get_subtitle_library_summary')
  },
}
