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
  network!: 'ws' | 'tcp' | 'http2' | 'grpc' | 'kcp'

  security?: 'tls' | 'none' | 'reality' | undefined

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
  wsSettings!: WebSocketProtocolSetting
}

class TcpProtocol extends ProtocolSetting {
  tcpSettings!: Record<string, any>
}

export class Http2ProtocolSetting {
  host!: string[]
  path!: string
}

class Http2Protocol extends ProtocolSetting {
  http2Settings!: Http2ProtocolSetting
}

class GrpcProtocol extends ProtocolSetting {
  grpcSettings!: Record<string, any>
}

class KcpProtocol extends ProtocolSetting {
  kcpSettings!: Record<string, any>
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
