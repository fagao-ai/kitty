import { Type } from 'class-transformer'

// enum Security {
//   tls,
//   none,
//   reality,
// }

class TLSSetting {
  allowInsecure!: boolean
  serverName!: string
}

class ProtocolSetting {
  network!: string

  security?: 'tls' | 'none' | 'reality' | undefined

  tlsSettings?: TLSSetting
}

class WebSocketHeader {
  host!: string
}

class WebSocketProtocolSetting {
  path!: string
  headers!: WebSocketHeader
}

class WebSocketProtocol extends ProtocolSetting {
  wsSettings!: WebSocketProtocolSetting
}

class TcpProtocol extends ProtocolSetting {
  tcpSettings!: Record<string, any>
}

class Http2ProtocolSetting {
  host!: string[]
  path!: string
}

class Http2Protocol extends ProtocolSetting {
  http2Settings!: Http2ProtocolSetting
}

class GrpcProtocolSetting {
  serviceName!: string

  multiMode!: boolean

  idle_time!: number

  healthCheckTimeout!: number

  permitWithoutStream!: boolean

  initialWindowsSize!: number
}

class GrpcProtocol extends ProtocolSetting {
  grpcSettings!: GrpcProtocolSetting
}

enum KcpType {
  none,
  srtp,
  utp,
  'wechat-video',
  dtls,
  wireguard,
  dns,
}

class KcpHeader {
  type!: KcpType
  domain!: string
}

class KcpProtocolSetting {
  mtu!: number

  tti!: number

  uplinkCapacity!: number

  downlinkCapacity!: number

  congestion!: number

  readBufferSize!: number

  sriteBufferSize!: number

  header!: KcpHeader

  seed?: string
}

class KcpProtocol extends ProtocolSetting {
  kcpSettings!: KcpProtocolSetting
}

type StreamSettings = WebSocketProtocol | TcpProtocol | Http2Protocol | GrpcProtocol | KcpProtocol

export class Xray {
  id!: number

  name!: string

  protocol!: string

  uuid!: string

  address!: string

  port!: number

  @Type(() => ProtocolSetting, {
    discriminator: {
      property: 'type',
      subTypes: [
        { value: WebSocketProtocol, name: 'WebSocket' },
        { value: TcpProtocol, name: 'Tcp' },
        { value: Http2Protocol, name: 'http2' },
        { value: GrpcProtocol, name: 'grpc' },
        { value: KcpProtocol, name: 'kcp' },
      ],
    },
  })
  streamSettings!: StreamSettings

  subscribeId?: number
}
