import { onUnmounted, unref, watch } from 'vue'
import { autoUpdateSubscription, batchGetSubscriptions } from '@/apis/proxy'
import { useTask } from '@/tools/useTask'
import { settingStore } from '@/views/setting/store'

export function useSubscriptionAutoUpdate() {
  const hour = unref(settingStore).autoUpdate || 3
  const { startTask, stopTask, taskStatus } = useTask(hour, async () => {
    const subscriptions = await batchGetSubscriptions()
    await autoUpdateSubscription(subscriptions.map(item => item.id))
  })

  function autoUpdate() {
    startTask()
  }

  function stopAutoUpdate() {
    stopTask()
  }

  const unwatch = watch(settingStore, (val, oldVal) => {
    if (!oldVal || val.autoUpdate !== oldVal.autoUpdate)
      stopAutoUpdate()
    autoUpdate()
  }, { immediate: true })

  onUnmounted(() => {
    stopAutoUpdate()
    unwatch()
  })

  return { autoUpdate, stopAutoUpdate, updateStatus: taskStatus }
}
