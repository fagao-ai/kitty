import { autoUpdateSubscription, batchGetSubscriptions } from '@/apis/proxy'

export function useSubscriptionAutoUpdate() {
  let intervalId: any

  function autoUpdate(t: number) {
    intervalId = setInterval(async () => {
      const subscriptions = await batchGetSubscriptions()
      await autoUpdateSubscription(subscriptions.map(item => item.id))
    }, t * 3600 * 1000)
  }

  function stopAutoUpdate() {
    clearInterval(intervalId)
  }

  return { autoUpdate, stopAutoUpdate }
}
