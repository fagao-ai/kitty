import { useStorage } from '@vueuse/core'
import type { Subscription } from '@/types/proxy'

const defaultSubscription: Subscription = { id: 0, url: '' }

const subscriptionStore = useStorage('subscription', defaultSubscription)

export { subscriptionStore }
