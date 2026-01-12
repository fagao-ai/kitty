import type { HumpsProcessorParameter } from 'humps'
import { camelizeKeys, decamelizeKeys } from 'humps'
import { instanceToPlain, plainToInstance } from 'class-transformer'
import { Xray } from '@/models/xray'
import { invoke } from '@/utils/invoke'
import type { HysteriaProxy, ImportProxy, ProxyDelay, ProxyDelayInfo, Subscription, XrayProxy } from '@/types/proxy'
import { ProxyType } from '@/types/proxy'

export async function getAllHysterias() {
  const res = await invoke<HysteriaProxy[]>('get_all_hysterias')
  return camelizeKeys(res.data) as HysteriaProxy[]
}

export async function getHysteriaById(id: number) {
  const res = await invoke<HysteriaProxy>('get_hysteria_by_id', { id })
  return camelizeKeys(res.data) as HysteriaProxy | null
}

export async function getXrayById(id: number) {
  const res = await invoke<XrayProxy>('get_xray_by_id', { id })
  if (!res.data)
    return null
  const data = camelizeKeys<XrayProxy>(res.data, (key: string, _: HumpsProcessorParameter): string => {
    if (key === 'Host')
      return 'host'
    return key
  })
  return data
}

export async function getProxyByIdAndType(id: number, proxyType: ProxyType) {
  switch (proxyType) {
    case ProxyType.Hysteria:
      return await getHysteriaById(id)
    case ProxyType.Xray:
      return await getXrayById(id)
  }
}

export async function createXrayProxy(xrayForm: XrayProxy) {
  const groupName = xrayForm.streamSettings.network
  const formCopy = { ...xrayForm }
  const record = instanceToPlain(plainToInstance(Xray, formCopy, { groups: [groupName] }), { groups: [groupName] })
  await invoke('add_xray_item', { record })
}

export async function createHysteriaProxy(hysteriaForm: HysteriaProxy) {
  await invoke('add_hysteria_item', { record: decamelizeKeys(hysteriaForm) })
}

export async function getAllXraies() {
  const res = await invoke<XrayProxy[]>('get_all_xrays')
  return camelizeKeys(res.data) as XrayProxy[]
}

export async function createImportProxy(importProxyForm: ImportProxy) {
  await invoke('import_xray_subscribe', { url: importProxyForm.url })
}

export async function updateXrayProxy(xrayForm: XrayProxy) {
  const groupName = xrayForm.streamSettings.network
  const formCopy = { ...xrayForm }
  const record = instanceToPlain(plainToInstance(Xray, formCopy, { groups: [groupName] }), { groups: [groupName] })
  await invoke('update_xray_item', { record })
}

export async function updateHysteriaProxy(hysteriaForm: HysteriaProxy) {
  await invoke('update_hysteria_item', { record: decamelizeKeys(hysteriaForm) })
}

export async function autoUpdateSubscription(subscriptionIds: number[]) {
  await invoke('refresh_xray_subscription', { record_ids: subscriptionIds })
}

export async function batchGetSubscriptions(): Promise<Subscription[]> {
  const res = await invoke<Subscription[]>('batch_get_subscriptions')
  return res.data
}

export async function xrayProxiedDelay(proxies: ProxyDelayInfo[]) {
  const res = await invoke<ProxyDelay[]>('proxies_delay_test', { proxies })

  return res.data.reduce((acc, item) => {
    acc[item.id] = item.delay
    return acc
  }, {} as Record<number, number>)
}

export async function currentProxyDelay(proxy: string, targetUrl: string) {
  const res = await invoke<number>('test_current_proxy', { proxy, target_url: targetUrl })

  return res.data
}

export async function setProxy(enable: boolean, _id: number | null = null) {
  if (enable) {
    // Server is already started by app auto-start, just set system proxy
    await invoke('set_system_proxy_only')
  }
  else {
    await invoke('stop_system_proxy')
  }
}
