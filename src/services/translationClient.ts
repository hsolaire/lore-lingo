import type { ActiveSessionSummary, LatestTranslationSummary } from '@/types/translation'
import { invokeCommand, listenEvent } from './tauriClient'

export const translationClient = {
  activeSession(): Promise<ActiveSessionSummary | null> {
    return invokeCommand<ActiveSessionSummary | null>('get_active_translation_session')
  },
  latestResult(): Promise<LatestTranslationSummary | null> {
    return invokeCommand<LatestTranslationSummary | null>('get_latest_translation_result')
  },
  onSessionChanged(handler: (session: ActiveSessionSummary | null) => void) {
    return listenEvent<ActiveSessionSummary | null>('translation:session-changed', (event) => {
      handler(event.payload)
    })
  },
}
