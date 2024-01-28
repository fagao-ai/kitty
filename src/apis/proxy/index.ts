import { camelizeKeys, decamelizeKeys } from 'humps'
import { instanceToPlain, plainToInstance } from 'class-transformer'
import { Xray } from '@/models/xray'
import { invoke } from '@/utils/invoke'
import type { HysteriaProxy, XrayProxy } from '@/types/proxy'

export async function getAllHysterias() {
  const res = await invoke<HysteriaProxy[]>('get_all_hysterias')
  return camelizeKeys(res.data) as HysteriaProxy[]
}

export async function createXrayProxy(xrayForm: XrayProxy) {
  const groupName = xrayForm.streamSettings.network
  const formCopy = { ...xrayForm }
  const record = instanceToPlain(plainToInstance(Xray, formCopy, { groups: [groupName] }), { groups: [groupName] })
  // console.log(record)
  await invoke('add_xray_item', { record })
}

export async function createHysteriaProxy(hysteriaForm: HysteriaProxy) {
  await invoke('add_hysteria_item', { record: decamelizeKeys(hysteriaForm) })
}

export async function getAllXraies() {
  const res = await invoke<XrayProxy[]>('get_all_xrays')
  return camelizeKeys(res.data) as XrayProxy[]
}
