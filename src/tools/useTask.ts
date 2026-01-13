import { ref } from 'vue'

function getTimeUntilNextRun(hour: number): number {
  const now = new Date()
  const nextRun = new Date(now.getFullYear(), now.getMonth(), now.getDate(), hour)

  if (nextRun < now)
    nextRun.setDate(nextRun.getDate() + 1)

  return nextRun.getTime() - now.getTime()
}

export function useTask(hour: number, taskFn: (...args: any[]) => Promise<void>) {
  const taskStatus = ref<'stop' | 'running'>('stop')
  let timeoutId: ReturnType<typeof setTimeout> | undefined

  async function runTask() {
    try {
      await taskFn()
    }
    catch {
      clearTimeout(timeoutId)
      taskStatus.value = 'stop'
      return
    }

    timeoutId = setTimeout(runTask, getTimeUntilNextRun(hour))
  }

  async function startTask() {
    clearTimeout(timeoutId)
    taskStatus.value = 'running'

    try {
      await taskFn()
    }
    catch {
      clearTimeout(timeoutId)
      taskStatus.value = 'stop'
      return
    }

    timeoutId = setTimeout(runTask, getTimeUntilNextRun(hour))
  }

  function stopTask() {
    clearTimeout(timeoutId)
    taskStatus.value = 'stop'
  }

  return {
    startTask,
    stopTask,
    taskStatus,
  }
}
