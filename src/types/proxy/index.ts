import type { Xray } from '@/models/xray'

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
  id?: number
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

export interface ProxyCard {
  id: number
  type: ProxyType
  tag: string
  name: string
  delay: number
  protocol: string
  source: 'subscription' | 'manual'  // Node source: subscription or manual
  isActive?: boolean          // Whether this proxy is currently active
}

export interface ImportProxy {
  id?: number
  url: string
}

export type Subscription = Required<ImportProxy>

export interface ProxyDelay {
  id: number
  delay: number
}

export interface ProxyDelayInfo {
  id: number
  address: string
  port: number
  proxy_type: 'Xray' | 'Hysteria2'  // Must match Rust enum variants
}
