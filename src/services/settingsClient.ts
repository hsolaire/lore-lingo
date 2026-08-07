import type { SettingsSummary, TranslationSettingsSummary } from '@/types/settings'
import { invokeCommand } from './tauriClient'

export const settingsClient = {
  get(): Promise<SettingsSummary> {
    return invokeCommand<SettingsSummary>('get_settings')
  },
  saveTranslation(input: TranslationSettingsSummary): Promise<SettingsSummary> {
    return invokeCommand<SettingsSummary>('save_translation_settings', { input })
  },
}
