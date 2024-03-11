<script setup lang="ts">
import { type WatchStopHandle, nextTick, onMounted, onUnmounted, ref, watchEffect } from 'vue'
import type { LogInst } from 'naive-ui'
import { useI18n } from 'vue-i18n'
import { type UnlistenFn, listen } from '@tauri-apps/api/event'
import { useLogQueue } from '@/views/log/store'

const { t } = useI18n()

const logInstRef = ref<LogInst | null>(null)

let unlisten: UnlistenFn | undefined
let unwatch: WatchStopHandle | undefined
const { enqueueLog, logQueue } = useLogQueue(1000)

onMounted(async () => {
  unlisten = await listen<string>('kitty_logger', (event) => {
    enqueueLog(event.payload)
  })
  unwatch = watchEffect(() => {
    if (logQueue.value.length > 0) {
      nextTick(() => {
        logInstRef.value?.scrollTo({ position: 'bottom', silent: true })
      })
    }
  })
})

onUnmounted(() => {
  unlisten?.()
  unwatch?.()
})
</script>

<template>
  <div class="flex w-full h-full flex-col space-y-3">
    <div class="h-8 flex justify-between items-center">
      <div class="text-primay text-2xl font-extrabold">
        {{ t('menubar.logs') }}
      </div>
    </div>
    <div class="flex-1 overflow-y-auto max-w-full h-full">
      <n-log ref="logInstRef" class="w-full h-full" :lines="logQueue" :rows="35" language="kitty-log" trim />
    </div>
  </div>
</template>
