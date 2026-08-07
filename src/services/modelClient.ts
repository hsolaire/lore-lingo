import type { ModelProfileSummary } from '@/types/settings'
import { invokeCommand } from './tauriClient'

export const modelClient = {
  list(): Promise<ModelProfileSummary[]> {
    return invokeCommand<ModelProfileSummary[]>('list_model_profiles')
  },
}
