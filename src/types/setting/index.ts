export interface KittyBaseConfig {
  id: number
  httpPort: number
  socksPort: number
  delayTestUrl: string
  sysproxyFlag: boolean
  autoUpdate: number
  autoStart: boolean
  startAtLogin?: boolean
  language?: string
  allowLan?: boolean
  mode?: 'Global' | 'Rules' | 'Direct'
}
