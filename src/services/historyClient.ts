import type { HistorySummary } from '@/types/history'
import { invokeCommand } from './tauriClient'

export const historyClient = {
  summary(): Promise<HistorySummary> {
    return invokeCommand<HistorySummary>('get_history_summary')
  },
}
