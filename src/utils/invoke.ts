import { useMessage } from 'naive-ui'
import { invoke as tauriInvoke } from '@tauri-apps/api/primitives'
import type { InvokeArgs, InvokeOptions } from '@tauri-apps/api/types/primitives'
import type { KittyResponse } from '@/types'

const message = useMessage()

export async function invoke<T>(cmd: string, args?: InvokeArgs, options?: InvokeOptions) {
  try {
    if (import.meta.env.KITTY_ENV !== 'web')
      return await tauriInvoke<KittyResponse<T>>(cmd, args, options)

    const fetchOptions = {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(args ?? {}),
    }
    const resp = await fetch(`/api/${cmd}`, fetchOptions)
    return resp.json() as unknown as KittyResponse<T>
  }
  catch (e) {
    message.error(String(e))
    console.error(e)
    throw e
  }
}
