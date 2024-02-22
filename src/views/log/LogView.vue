<script setup lang="ts">
import { nextTick, onMounted, ref, watch, watchEffect } from 'vue'
import type { LogInst } from 'naive-ui'

function log() {
  const l: string[] = []
  for (let i = 0; i < 40; ++i)
    l.push(Math.random().toString(16))

  return `${l.join('\n')}\n`
}

const realtimeUpdate = ref(false)
const logRef = ref(log())
const logInstRef = ref<LogInst | null>(null)

const startRef = ref(false)
const timerRef = ref<number | null>(null)
function startRealtime() {
  startRef.value = !startRef.value
  if (startRef.value) {
    timerRef.value = window.setInterval(() => {
      logRef.value = logRef.value + log()
    }, 1000)
  }
  else if (timerRef.value) {
    clearInterval(timerRef.value)
    timerRef.value = null
  }
}
watch(realtimeUpdate, startRealtime)
onMounted(() => {
  watchEffect(() => {
    if (logRef.value) {
      nextTick(() => {
        logInstRef.value?.scrollTo({ position: 'bottom', silent: true })
      })
    }
  })
})
</script>

<template>
  <div class="flex w-full h-full flex-col space-y-3">
    <div class="h-8 flex justify-between items-center">
      <div class="text-primay text-2xl font-extrabold">
        Logs
      </div>
      <div class="flex justify-center items-center space-x-3">
        <n-icon
          size="32"
          class="text-primay"
          :class="realtimeUpdate ? 'cursor-not-allowed' : 'cursor-pointer'"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            xmlns:xlink="http://www.w3.org/1999/xlink"
            viewBox="0 0 24 24"
          >
            <path
              d="M17.65 6.35A7.958 7.958 0 0 0 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08A5.99 5.99 0 0 1 12 18c-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"
              fill="currentColor"
            />
          </svg>
        </n-icon>
        <n-checkbox v-model:checked="realtimeUpdate">
          实时更新
        </n-checkbox>
      </div>
    </div>
    <div class="flex-1 overflow-y-auto max-w-full h-full">
      <n-log
        ref="logInstRef"
        class="w-full h-full"
        :log="logRef"
        :rows="35"
        language="naive-log"
        trim
      />
    </div>
  </div>
</template>

<style lang="scss" scoped>
// :deep(.n-log) {
//   height: 100% !important;
// }
</style>
