import { useStorage, useSessionStorage } from '@vueuse/core'
import type { Subscription } from '@/types/proxy'

const defaultSubscription: Subscription = { id: 0, url: '' }

const subscriptionStore = useStorage('subscription', defaultSubscription)

const logStore = useSessionStorage<string[]>('log', [])

export { subscriptionStore, logStore }
