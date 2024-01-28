import 'reflect-metadata'
import { Exclude, Expose, Type } from 'class-transformer'

class TLSSetting {
  @Expose({ name: 'allowInsecure', toPlainOnly: true })
  allowInsecure!: boolean

  @Expose({ name: 'serverName', toPlainOnly: true })
  serverName!: string
}

class ProtocolSetting {
  @Expose()
  network!: 'ws' | 'tcp' | 'http2' | 'grpc' | 'kcp'

  @Expose()
  security?: 'tls' | 'none' | 'reality' | undefined

  @Expose({ name: 'tls_settings', toPlainOnly: true })
  tlsSettings?: TLSSetting
}

class WebSocketHeader {
  @Expose()
  host!: string
}

export class WebSocketProtocolSetting {
  @Expose()
  path!: string

  @Expose()
  headers!: WebSocketHeader
}

class WebSocketProtocol extends ProtocolSetting {
  declare network: 'ws'

  @Expose({ name: 'ws_settings', toPlainOnly: true })
  wsSettings!: WebSocketProtocolSetting
}

class TcpProtocol extends ProtocolSetting {
  declare network: 'tcp'

  @Expose({ name: 'tcp_settings', toPlainOnly: true })
  tcpSettings!: Record<string, any>
}

export class Http2ProtocolSetting {
  @Expose()
  host!: string[]

  @Expose()
  path!: string
}

class Http2Protocol extends ProtocolSetting {
  declare network: 'http2'

  @Expose({ name: 'http2_settings', toPlainOnly: true })
  http2Settings!: Http2ProtocolSetting
}

class GrpcProtocol extends ProtocolSetting {
  declare network: 'grpc'

  @Expose({ name: 'grpc_settings', toPlainOnly: true })
  grpcSettings!: Record<string, any>
}

class KcpProtocol extends ProtocolSetting {
  declare network: 'kcp'

  @Expose({ name: 'kcp_settings', toPlainOnly: true })
  kcpSettings!: Record<string, any>
}

class TrojanProtocol extends ProtocolSetting {
  declare network: 'tcp'
}

type StreamSettings = WebSocketProtocol | TcpProtocol | Http2Protocol | GrpcProtocol | KcpProtocol | TrojanProtocol

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

  @Type(() => ProtocolSetting, {
    discriminator: {
      property: 'network',
      subTypes: [
        { value: WebSocketProtocol, name: 'ws' },
        { value: TcpProtocol, name: 'tcp' },
        { value: Http2Protocol, name: 'http2' },
        { value: GrpcProtocol, name: 'grpc' },
        { value: KcpProtocol, name: 'kcp' },
      ],
    },
  })
  @Expose({ name: 'stream_settings', toPlainOnly: true })
  streamSettings!: StreamSettings
}
