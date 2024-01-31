export interface KittyBaseConfig {
  id: number
  httpPort: number
  socksPort: number
  delayTestUrl: string
  startAtLogin?: boolean
  language?: string
  allowLan?: boolean
  mode?: 'Global' | 'Rules' | 'Direct'
}
