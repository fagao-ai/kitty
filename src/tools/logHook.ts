import { customRef } from 'vue'

export function useQueueRef<T>(size: number = 1000) {
  return customRef((track, trigger) => {
    const queue: T[] = []
    return {
      get() {
        track()
        return queue
      },
      set(value) {
        if (queue.length >= size)
          queue.shift()

        const item = value.at(-1)
        if (!item)
          return
        queue.push(item)
        trigger()
      },
    }
  })
}
