import { invoke as tauriInvoke } from '@tauri-apps/api/primitives'
import type { InvokeArgs, InvokeOptions } from '@tauri-apps/api/types/primitives'

export async function invoke<T>(cmd: string, args?: InvokeArgs, options?: InvokeOptions) {
  if (import.meta.env.KITTY_ENV !== 'web')
    return await tauriInvoke<T>(cmd, args, options)

  const fetchOptions = {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(args ?? {}),
  }
  const resp = await fetch(`/api/${cmd}`, fetchOptions)
  return resp.json() as T
}
