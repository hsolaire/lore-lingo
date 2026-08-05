import { invoke } from '@tauri-apps/api/core'
import { listen, type Event, type UnlistenFn } from '@tauri-apps/api/event'

export function invokeCommand<TResult>(command: string, args?: Record<string, unknown>): Promise<TResult> {
  return invoke<TResult>(command, args)
}

export function listenEvent<TPayload>(
  eventName: string,
  handler: (event: Event<TPayload>) => void,
): Promise<UnlistenFn> {
  return listen<TPayload>(eventName, handler)
}
