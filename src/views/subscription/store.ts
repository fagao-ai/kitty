import { ref } from 'vue'
import type { SubscriptionInfo } from '@/types/subscription'

export const subscriptionStore = ref<{
  subscriptions: SubscriptionInfo[]
  loading: boolean
}>({
  subscriptions: [],
  loading: false,
})
