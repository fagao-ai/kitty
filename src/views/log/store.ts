import { useSessionStorage } from '@vueuse/core'

export const logStore = useSessionStorage<string[]>('log', [])

export function useLogQueue(size = 1000) {
  const enqueueLog = (log: string) => {
    if (logStore.value.length >= size)
      logStore.value.shift()
    logStore.value.push(log)
  }

  const clearLog = () => {
    logStore.value = []
  }

  return { enqueueLog, clearLog, logQueue: logStore }
}
