<script
  setup
  lang="ts"
>
import { type WatchStopHandle, nextTick, onMounted, onUnmounted, ref, watchEffect } from 'vue'
import type { LogInst } from 'naive-ui'
import { useI18n } from 'vue-i18n'
import { type UnlistenFn, listen } from '@tauri-apps/api/event'
import { useLogQueue } from '@/views/log/store'
import HeaderBar from '@/components/HeaderBar.vue'

const { t } = useI18n()

const logInstRef = ref<LogInst | null>(null)

let unlisten: UnlistenFn | undefined
let unwatch: WatchStopHandle | undefined
let eventCount = 0
const { enqueueLog, logQueue } = useLogQueue(1000)

onMounted(async () => {
  console.log('[LogView] Setting up kitty_logger listener')
  unlisten = await listen<string | string[]>('kitty_logger', (event) => {
    eventCount++
    // Handle both single string (old format) and array (new batch format)
    const logs = Array.isArray(event.payload) ? event.payload : [event.payload]
    for (const log of logs) {
      enqueueLog(log)
    }
    if (eventCount <= 10 || eventCount % 50 === 0) {
      console.log(`[LogView] Batch #${eventCount}: ${logs.length} logs`)
    }
  })
  unwatch = watchEffect(() => {
    if (logQueue.value.length > 0) {
      nextTick(() => {
        logInstRef.value?.scrollTo({ position: 'bottom', silent: true })
      })
    }
  })
  console.log('[LogView] Listener set up complete')
})

onUnmounted(() => {
  console.log(`[LogView] Unmounting, total events received: ${eventCount}`)
  unlisten?.()
  unwatch?.()
})
</script>

<template>
  <div class="flex w-full h-full flex-col">
    <header-bar @toggle-mobile-menu="$emit('toggle-mobile-menu')">
      <template #mobile-menu-button>
        <n-icon size="24">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M3 12h18M3 6h18M3 18h18" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </n-icon>
      </template>
      <template #title>
        {{ t('menubar.logs') }}
      </template>
    </header-bar>
    <div class="flex-1 overflow-y-auto max-w-full h-full text-text-secondary dark:text-text-secondary">
      <n-log
        ref="logInstRef"
        class="w-full h-full"
        :lines="logQueue"
        :rows="35"
        language="kitty-log"
        trim
      />
    </div>
  </div>
</template>

<style
  lang="scss"
  scoped
>
:deep(.n-log) {
  user-select: text;
  -webkit-user-select: text;
}
</style>
