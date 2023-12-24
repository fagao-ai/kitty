import { invoke as tauriInvoke } from '@tauri-apps/api/primitives'
import { camelizeKeys } from 'humps'
import { useMessage } from 'naive-ui'
import type { InvokeArgs, InvokeOptions } from '@tauri-apps/api/types/primitives'
import type { KittyResponse } from '@/types'

export async function invoke<T>(cmd: string, args?: InvokeArgs, options?: InvokeOptions): Promise<KittyResponse<T>> {
  const message = useMessage()

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
    message.error(`${e}`, { duration: 3000 })
    console.error('kitty error', e)

    throw e
  }
  finally {
    // let permissionGranted = await isPermissionGranted()
    // if (!permissionGranted) {
    //   const permission = await requestPermission()
    //   permissionGranted = permission === 'granted'
    // }
    // if (permissionGranted) {
    //   sendNotification('Tauri is awesome!')
    //   sendNotification({ title: 'TAURI', body: 'Tauri is awesome!' })
    // }
  }
}
