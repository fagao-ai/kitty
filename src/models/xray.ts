import 'reflect-metadata'
import { Expose, Type } from 'class-transformer'

class TLSSetting {
  @Expose({ name: 'allowInsecure' })
  allowInsecure!: boolean

  @Expose({ name: 'serverName' })
  serverName!: string
}

class ProtocolSetting {
  network!: 'ws' | 'tcp' | 'http2' | 'grpc' | 'kcp'

  security?: 'tls' | 'none' | 'reality' | undefined

  @Expose({ name: 'tls_settings' })
  tlsSettings?: TLSSetting
}

class WebSocketHeader {
  host!: string
}

export class WebSocketProtocolSetting {
  path!: string
  headers!: WebSocketHeader
}

class WebSocketProtocol extends ProtocolSetting {
  declare network: 'ws'

  @Expose({ name: 'ws_settings' })
  wsSettings!: WebSocketProtocolSetting
}

class TcpProtocol extends ProtocolSetting {
  declare network: 'tcp'

  @Expose({ name: 'tcp_settings' })
  tcpSettings!: Record<string, any>
}

export class Http2ProtocolSetting {
  host!: string[]
  path!: string
}

class Http2Protocol extends ProtocolSetting {
  declare network: 'http2'

  @Expose({ name: 'http2_settings' })
  http2Settings!: Http2ProtocolSetting
}

class GrpcProtocol extends ProtocolSetting {
  declare network: 'grpc'

  @Expose({ name: 'grpc_settings' })
  grpcSettings!: Record<string, any>
}

class KcpProtocol extends ProtocolSetting {
  declare network: 'kcp'

  @Expose({ name: 'kcp_settings' })
  kcpSettings!: Record<string, any>
}

class TrojanProtocol extends ProtocolSetting {
  declare network: 'tcp'
}

type StreamSettings = WebSocketProtocol | TcpProtocol | Http2Protocol | GrpcProtocol | KcpProtocol | TrojanProtocol

export class Xray {
  id!: number

  name!: string

  protocol!: 'vless' | 'vmess' | 'trojan'

  uuid!: string

  address!: string

  port!: number

  @Type(() => ProtocolSetting, {
    discriminator: {
      property: 'stream_settings.network',
      subTypes: [
        { value: WebSocketProtocol, name: 'ws' },
        { value: TcpProtocol, name: 'tcp' },
        { value: Http2Protocol, name: 'http2' },
        { value: GrpcProtocol, name: 'grpc' },
        { value: KcpProtocol, name: 'kcp' },
      ],
    },
  })
  @Expose({ name: 'stream_settings' })
  streamSettings!: StreamSettings
}
