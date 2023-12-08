export interface ProxyCard {
  tag: string
  name: string
  delay: number
  protocol: string
}

interface BandWidth {
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
  serverAddress: string
  auth: string
  bandWidth: BandWidth
  tls: TLS
  socks5: Listener
  http: Listener
}

export interface ProxyAdd {
  showModal: boolean
  formData: HysteriaProxy
}
