export interface SubscriptionInfo {
  id: number
  name: string
  url: string
  isActive: boolean
  nodeCount: number
  createdAt: string
  updatedAt: string
  lastSyncAt?: string
}
