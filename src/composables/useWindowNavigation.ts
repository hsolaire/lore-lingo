import { windowClient } from '@/services/windowClient'

export type AppWindowLabel = 'main' | 'subtitle' | 'settings'

export function useWindowNavigation(currentWindow: AppWindowLabel) {
  return {
    openMain: () => windowClient.showOrFocus('main'),
    openSubtitle: () => windowClient.showOrFocus('subtitle'),
    openSettings: () => windowClient.showOrFocus('settings'),
    hideCurrent: () => windowClient.hide(currentWindow),
  }
}
