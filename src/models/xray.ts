import { Transform, Type } from 'class-transformer'

enum Security {
  tls,
  none,
  reality,
}

class TLSSetting {
  allowInsecure!: boolean
  serverName!: string
}

class ProtocolSetting {
  network!: string

  security?: Security

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

enum TcpType {
  none,
  http,
}

class TcpRequestHeader {
  @Transform(params => params.obj?.Host ?? [] as string[])
  host!: string[]

  @Transform(params => params.obj?.['User-Agent'] ?? [] as string[])
  userAgent!: string[]

  @Transform(params => params.obj?.['Accept-Encoding'] ?? [] as string[])
  acceptEncoding!: string[]

  @Transform(params => params.obj?.Connection ?? [] as string[])
  connection!: string[]

  @Transform(params => params.obj?.Pragma ?? '' as string)
  pragma!: string
}

class TcpRequest {
  version!: string
  method!: string
  path!: string
  headers!: TcpRequestHeader
}

class TcpResponseHeader {
  @Transform((params: any) => params.obj?.['Content-Type'] ?? [])
  contentType!: string[]

  @Transform(params => params.obj?.['transfer-Encoding'] ?? [])
  transferEncoding!: string[]

  @Transform(params => params.obj?.Connection ?? [])
  connection!: string[]

  @Transform(pragma => pragma.obj?.Pragma ?? '')
  pragma!: string
}

class TcpResponse {
  version!: string
  status!: number
  reason!: string
  headers!: TcpResponseHeader
}

class TcpHeader {
  type!: TcpType
  request?: TcpRequest
  response?: TcpResponse
}
class TcpProtocolSetting {
  header!: TcpHeader
}

class TcpProtocol extends ProtocolSetting {
  tcpSettings!: TcpProtocolSetting
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
