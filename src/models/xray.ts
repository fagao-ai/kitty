import { Exclude, Expose, Type } from 'class-transformer'

class TLSSetting {
  @Expose({ name: 'allowInsecure', toPlainOnly: true })
  allowInsecure!: boolean

  @Expose({ name: 'serverName', toPlainOnly: true })
  serverName!: string
}

class WebSocketHeader {
  @Expose({ name: 'Host', toPlainOnly: true })
  host!: string
}

export class WebSocketProtocolSetting {
  @Expose()
  path!: string

  @Type(() => WebSocketHeader)
  @Expose()
  headers!: WebSocketHeader
}

export class Http2ProtocolSetting {
  @Expose()
  host!: string[]

  @Expose()
  path!: string
}

class StreamSettings {
  @Expose()
  network!: 'ws' | 'tcp' | 'http2' | 'grpc' | 'kcp'

  @Expose()
  security?: 'tls' | 'none' | 'reality' | undefined

  @Expose({ name: 'tls_settings', toPlainOnly: true })
  tlsSettings?: TLSSetting

  @Type(() => WebSocketProtocolSetting)
  @Expose({ name: 'ws_settings', toPlainOnly: true, groups: ['ws'] })
  wsSettings!: WebSocketProtocolSetting

  @Expose({ name: 'tcp_settings', toPlainOnly: true, groups: ['tcp'] })
  tcpSettings!: Record<string, any>

  @Type(() => Http2ProtocolSetting)
  @Expose({ name: 'http2_settings', toPlainOnly: true, groups: ['http2'] })
  http2Settings!: Http2ProtocolSetting

  @Expose({ name: 'grpc_settings', toPlainOnly: true, groups: ['grpc'] })
  grpcSettings!: Record<string, any>

  @Expose({ name: 'kcp_settings', toPlainOnly: true, groups: ['kcp'] })
  kcpSettings!: Record<string, any>
}

export class Xray {
  @Exclude({ toPlainOnly: true })
  id!: number

  @Expose()
  name!: string

  @Expose()
  protocol!: 'vless' | 'vmess' | 'trojan'

  @Expose()
  uuid!: string

  @Expose()
  address!: string

  @Expose()
  port!: number

  @Type(() => StreamSettings)
  @Expose({ name: 'stream_settings', toPlainOnly: true })
  streamSettings!: StreamSettings
}

// export class XrayController {
//   static getForm() {
//     const tlsSettings = new TLSSetting()
//     tlsSettings.allowInsecure = true
//     tlsSettings.serverName = ''

//     const wsHeader = new WebSocketHeader()
//     wsHeader.host = ''

//     const wsSettings = new WebSocketProtocolSetting()
//     wsSettings.path = ''
//     wsSettings.headers = wsHeader

//     const http2Setting = new Http2ProtocolSetting()
//     http2Setting.host = ['']
//     http2Setting.path = ''

//     const streamSettings = new StreamSettings()
//     streamSettings.network = 'ws'
//     streamSettings.security = 'none'
//     streamSettings.tlsSettings = tlsSettings
//     streamSettings.wsSettings = wsSettings
//     streamSettings.grpcSettings = {}
//     streamSettings.http2Settings = http2Setting
//     streamSettings.kcpSettings = {}
//     streamSettings.tcpSettings = {}

//     const xray = new Xray()
//     xray.id = 0
//     xray.name = ''
//     xray.protocol = 'vmess'
//     xray.uuid = ''
//     xray.address = ''
//     xray.port = 443
//     xray.streamSettings = streamSettings

//     return xray
//   }
// }
