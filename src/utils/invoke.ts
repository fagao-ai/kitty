import { useMessage } from 'naive-ui'
import { invoke as tauriInvoke } from '@tauri-apps/api/primitives'
import { camelizeKeys } from 'humps'
import type { InvokeArgs, InvokeOptions } from '@tauri-apps/api/types/primitives'
import type { KittyResponse } from '@/types'

const message = useMessage()

export async function invoke<T>(cmd: string, args?: InvokeArgs, options?: InvokeOptions): Promise<KittyResponse<T>> {
  try {
    if (import.meta.env.KITTY_ENV !== 'web') {
      const resp = await tauriInvoke<KittyResponse<T>>(cmd, args, options)
      return camelizeKeys<KittyResponse<T>>(resp) as KittyResponse<T>
    }

    const fetchOptions = {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(args ?? {}),
    }
    const resp = await fetch(`/api/${cmd}`, fetchOptions)
    return camelizeKeys(resp.json()) as unknown as KittyResponse<T>
  }
  catch (e) {
    message.error(String(e))
    console.error('kitty error', e)
    throw e
  }
}
