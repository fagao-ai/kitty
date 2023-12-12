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

interface Listener {
  listen: string
}

export interface HysteriaProxy {
  server: string
  auth: string
  bandwidth: Bandwidth
  tls: TLS
}

export interface ProxyAdd {
  showModal: boolean
  formData: HysteriaProxy
}
