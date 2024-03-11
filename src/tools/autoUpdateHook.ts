import { autoUpdateSubscription, batchGetSubscriptions } from '@/apis/proxy'
import { useTask } from '@/tools/useTask'

export function useSubscriptionAutoUpdate() {
  const { startTask, stopTask } = useTask(3, async () => {
    const subscriptions = await batchGetSubscriptions()
    await autoUpdateSubscription(subscriptions.map(item => item.id))
  })

  function autoUpdate() {
    startTask()
  }

  function stopAutoUpdate() {
    stopTask()
  }

  return { autoUpdate, stopAutoUpdate }
}
