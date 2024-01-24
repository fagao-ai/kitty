import { camelizeKeys, decamelizeKeys } from 'humps'
import { invoke } from '@/utils/invoke'
import type { ProxyCard as Card, HysteriaProxy, XrayProxy } from '@/types/proxy'

export async function getAllHysterias() {
  const res = await invoke<Card[]>('get_all_hysterias')
  return res.data.map(item => ({
    tag: 'hysteria',
    name: item.name,
    delay: 200,
    protocol: 'TCP',
  }))
  // cards.value.push({
  //   tag: 'hysteria',
  //   name: 'test',
  //   delay: 200,
  //   protocol: 'TCP',
  // })
}

export async function createXrayProxy(xrayForm: XrayProxy) {
  const res = decamelizeKeys(xrayForm) as any
  const tls_settings = res.stream_settings.tls_settings
  res.streamSettings = res.stream_settings
  res.streamSettings.tlsSettings = { allowInsecure: tls_settings.allow_insecure, serverName: tls_settings.server_name }
  await invoke('add_xray_item', { record: res })
}

export async function createHysteriaProxy(hysteriaForm: HysteriaProxy) {
  await invoke('add_hy_item', { record: decamelizeKeys(hysteriaForm) })
}

export async function getAllXraies() {
  const res = await invoke<XrayProxy[]>('get_all_xrays')
  return camelizeKeys(res.data) as XrayProxy[]
}
