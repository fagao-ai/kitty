import { camelizeKeys, decamelizeKeys } from 'humps'
import { instanceToPlain, plainToInstance } from 'class-transformer'
import { Xray } from '@/models/xray'
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
  const formCopy = { ...xrayForm }
  // const formCopy: XrayProxy = {
  //   id: 0,
  //   name: '1',
  //   protocol: 'trojan',
  //   uuid: '11',
  //   address: '1',
  //   port: 1,
  //   streamSettings: {
  //     network: 'ws',
  //     security: 'none',
  //     wsSettings: {
  //       path: '/ws',
  //       headers: {
  //         host: '1',
  //       }
  //     },
  //   },
  // }
  // const instance = plainToInstance(Xray, formCopy)
  // const plain = instanceToPlain(instance)
  // console.log('instance', instance, '\nplain', plain)
  await invoke('add_xray_item', { record: instanceToPlain(plainToInstance(Xray, formCopy)) })
}

export async function createHysteriaProxy(hysteriaForm: HysteriaProxy) {
  await invoke('add_hysteria_item', { record: decamelizeKeys(hysteriaForm) })
}

export async function getAllXraies() {
  const res = await invoke<XrayProxy[]>('get_all_xrays')
  return camelizeKeys(res.data) as XrayProxy[]
}
