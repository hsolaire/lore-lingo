import { readonly, ref } from 'vue'

import { appClient } from '@/services/appClient'
import type { BootstrapState } from '@/types/app'
import { toPublicError } from '@/types/translation'

export function useBootstrap() {
  const state = ref<BootstrapState>({ status: 'loading', snapshot: null, error: null })

  async function load() {
    state.value = { status: 'loading', snapshot: null, error: null }
    try {
      const snapshot = await appClient.getBootstrapSnapshot()
      state.value = snapshot.platform.platform === 'unsupported'
        ? {
            status: 'unsupported',
            snapshot,
            error: '当前平台尚未提供完整的桌面能力。',
          }
        : { status: 'ready', snapshot, error: null }
    } catch (reason: unknown) {
      const error = toPublicError(reason)
      state.value = error.code === 'unsupported'
        ? { status: 'unsupported', snapshot: null, error: error.message }
        : { status: 'database_error', snapshot: null, error: error.message }
    }
  }

  void load()

  return {
    bootstrap: readonly(state),
    reloadBootstrap: load,
  }
}
