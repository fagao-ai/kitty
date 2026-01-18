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
  let timeoutId: NodeJS.Timeout | undefined

  async function runTask() {
    taskStatus.value = 'running'
    try {
      await taskFn()
    }
    catch {
      taskStatus.value = 'stop'
      return
    }

    // Task completed, set status to stop and schedule next run
    taskStatus.value = 'stop'
    timeoutId = setTimeout(runTask, getTimeUntilNextRun(hour))
  }

  async function startTask() {
    clearTimeout(timeoutId)
    await runTask()
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
