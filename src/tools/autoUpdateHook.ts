import { onUnmounted, unref, watch } from 'vue'
import { autoRefreshActiveSubscription } from '@/apis/proxy'
import { useTask } from '@/tools/useTask'
import { settingStore } from '@/views/setting/store'

export function useSubscriptionAutoUpdate() {
  const hour = unref(settingStore).autoUpdate || 3
  const { startTask, stopTask, taskStatus } = useTask(hour, async () => {
    // Auto-refresh active subscription with smart checking
    await autoRefreshActiveSubscription()
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
