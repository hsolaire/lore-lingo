import type { BootstrapSnapshot } from '@/types/app'
import { invokeCommand } from './tauriClient'

export const appClient = {
  getBootstrapSnapshot(): Promise<BootstrapSnapshot> {
    return invokeCommand<BootstrapSnapshot>('get_bootstrap_snapshot')
  },
}
