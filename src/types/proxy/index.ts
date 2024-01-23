import type { Xray } from '@/models/xray'

export interface ProxyCard {
  tag: string
  name: string
  delay: number
  protocol: string
}

interface Bandwidth {
  up: string
  down: string
}

interface TLS {
  sni: string
  insecure: boolean
}

// interface Listener {
//   listen: string
// }

export interface HysteriaProxy {
  name: string
  server: string
  auth: string
  bandwidth: Bandwidth
  tls: TLS
}

export type XrayProxy = {
  [K in keyof Xray]: Xray[K];
}

// export interface ProxyAdd {
//   showModal: boolean
//   hysteriaForm: HysteriaProxy
//   xrayForm: XrayProxy
// }

export enum ProxyType {
  Hysteria = 'hysteria',
  Xray = 'xray',
}
