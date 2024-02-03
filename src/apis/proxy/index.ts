import { camelizeKeys, decamelizeKeys } from 'humps'
import { instanceToPlain, plainToInstance } from 'class-transformer'
import { Xray } from '@/models/xray'
import { invoke } from '@/utils/invoke'
import { HysteriaProxy, ImportProxy, ProxyType, XrayProxy } from '@/types/proxy'

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
  if (!res.data) return null
  const data = camelizeKeys<XrayProxy>(res.data)
  if (data.streamSettings.network === "ws") {
    const headers = { ...data.streamSettings.wsSettings.headers } as any
    data.streamSettings.wsSettings.headers.host = headers.Host
    delete (data.streamSettings.wsSettings.headers as any).Host
  }
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
