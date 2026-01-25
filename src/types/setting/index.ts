export interface KittyBaseConfig {
  id: number
  localIp: string
  httpPort: number
  socksPort: number
  delayTestUrl: string
  sysproxyFlag: boolean
  autoStart: boolean
  language: string
  allowLan: boolean
  mode: 'Global' | 'Rules' | 'Direct'
  updateInterval: number
  logLevel: 'trace' | 'debug' | 'info' | 'warn' | 'error'
}
