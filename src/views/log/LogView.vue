<script
  setup
  lang="ts"
>
import { nextTick, onMounted, onUnmounted, ref, watchEffect } from 'vue'
import { useI18n } from 'vue-i18n'
import { type UnlistenFn, listen } from '@tauri-apps/api/event'
import { useLogQueue } from '@/views/log/store'
import HeaderBar from '@/components/HeaderBar.vue'
import VirtualScroller from 'primevue/virtualscroller'
import { highlight, languages } from 'highlight.js/lib/core'
import HighlightJS from 'highlight.js/lib/common'

const { t } = useI18n()

const logContainer = ref<HTMLElement | null>(null)

let unlisten: UnlistenFn | undefined
let unwatch: ReturnType<typeof watchEffect> | undefined
const { enqueueLog, logQueue } = useLogQueue(1000)

onMounted(async () => {
  unlisten = await listen<string>('kitty_logger', (event) => {
    enqueueLog(event.payload)
  })
  unwatch = watchEffect(() => {
    if (logQueue.value.length > 0 && logContainer.value) {
      nextTick(() => {
        if (logContainer.value) {
          logContainer.value.scrollTop = logContainer.value.scrollHeight
        }
      })
    }
  })
})

onUnmounted(() => {
  unlisten?.()
  unwatch?.()
})

function highlightLog(line: string): string {
  try {
    // Simple syntax highlighting for log output
    const timestamp = line.match(/^\[[\d:\-\.]+\]/)?.[0]
    const level = line.match(/\[(INFO|WARN|ERROR|DEBUG)\]/)?.[1]
    const message = line.substring(line.indexOf('] ') + 2)

    let highlighted = ''
    if (timestamp) {
      highlighted += `<span class="text-gray-500">${timestamp}</span> `
    }

    if (level) {
      const levelClass = {
        INFO: 'text-blue-400',
        WARN: 'text-yellow-400',
        ERROR: 'text-red-400',
        DEBUG: 'text-gray-400'
      }[level] || 'text-gray-400'
      highlighted += `<span class="${levelClass} font-bold">[${level}]</span> `
    }

    highlighted += `<span class="text-slate-600 dark:text-slate-300">${message}</span>`
    return highlighted
  }
  catch {
    return line
  }
}
</script>

<template>
  <div class="flex w-full h-full flex-col">
    <header-bar>
      <template #title>
        {{ t('menubar.logs') }}
      </template>
    </header-bar>
    <div
      ref="logContainer"
      class="flex-1 overflow-y-auto max-w-full h-full text-slate-600 dark:text-slate-300 bg-white dark:bg-gray-900 p-4 font-mono text-sm"
    >
      <VirtualScroller
        :items="logQueue"
        :item-size="[40, null]"
        class="w-full h-full"
      >
        <template #default="{ item }">
          <div
            class="whitespace-nowrap overflow-x-auto border-b border-gray-100 dark:border-gray-800"
            v-html="highlightLog(item)"
          />
        </template>
      </VirtualScroller>
    </div>
  </div>
</template>

<style
  lang="scss"
  scoped
>
:deep(.p-virtualscroller) {
  user-select: text;
  -webkit-user-select: text;
  height: 100%;
}
</style>
