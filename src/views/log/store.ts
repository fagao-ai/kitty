import { ref } from 'vue'

export function useLogQueue(size = 1000) {
  const logQueue = ref<string[]>([])

  const enqueueLog = (log: string) => {
    if (logQueue.value.length >= size)
      logQueue.value.shift()
    logQueue.value.push(log)
  }

  const clearLog = () => {
    logQueue.value = []
  }

  return { enqueueLog, clearLog, logQueue }
}
