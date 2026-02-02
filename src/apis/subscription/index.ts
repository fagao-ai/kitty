import { invoke } from '@/utils/invoke'
import type { SubscriptionInfo } from '@/types/subscription'

export async function getAllSubscriptions(): Promise<SubscriptionInfo[]> {
  const res = await invoke<SubscriptionInfo[]>('get_all_subscriptions')
  return res.data
}

export async function createSubscription(
  name: string,
  url: string,
): Promise<SubscriptionInfo> {
  const res = await invoke<SubscriptionInfo>('create_subscription', { name, url })
  return res.data
}

export async function updateSubscription(
  id: number,
  name: string,
  url: string,
): Promise<SubscriptionInfo> {
  const res = await invoke<SubscriptionInfo>('update_subscription', { id, name, url })
  return res.data
}

export async function deleteSubscription(id: number): Promise<void> {
  await invoke('delete_subscription', { id })
}

export async function switchSubscription(id: number): Promise<void> {
  await invoke('switch_subscription', { id })
}

export async function refreshSubscription(id: number): Promise<void> {
  await invoke('refresh_subscription', { id })
}
