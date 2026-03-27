<script
  setup
  lang="ts"
>
import hljs from 'highlight.js/lib/core'
import { type WatchStopHandle, nextTick, onMounted, onUnmounted, ref, watchEffect } from 'vue'
import { useI18n } from 'vue-i18n'
import { type UnlistenFn, listen } from '@tauri-apps/api/event'
import { useLogQueue } from '@/views/log/store'
import HeaderBar from '@/components/HeaderBar.vue'

defineEmits<{
  toggleMobileMenu: []
}>()

const { t } = useI18n()

const logContainerRef = ref<HTMLDivElement | null>(null)

let unlisten: UnlistenFn | undefined
let unwatch: WatchStopHandle | undefined
const { enqueueLog, logQueue } = useLogQueue(1000)

onMounted(async () => {
  unlisten = await listen<string | string[]>('kitty_logger', (event) => {
    const logs = Array.isArray(event.payload) ? event.payload : [event.payload]
    for (const log of logs) {
      enqueueLog(log)
    }
  })
  unwatch = watchEffect(() => {
    if (logQueue.value.length > 0) {
      nextTick(() => {
        if (logContainerRef.value) {
          logContainerRef.value.scrollTop = logContainerRef.value.scrollHeight
        }
      })
    }
  })
})

onUnmounted(() => {
  unlisten?.()
  unwatch?.()
})

function highlightLine(line: string) {
  return hljs.highlight(line.trim() ? line : ' ', { language: 'kitty-log' }).value
}
</script>

<template>
  <div class="flex w-full h-full flex-col">
    <header-bar @toggle-mobile-menu="$emit('toggleMobileMenu')">
      <template #mobile-menu-button>
        <span class="inline-flex h-6 w-6 items-center justify-center">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M3 12h18M3 6h18M3 18h18" stroke-linecap="round" stroke-linejoin="round" />
          </svg>
        </span>
      </template>
      <template #title>
        {{ t('menubar.logs') }}
      </template>
    </header-bar>
    <div
      ref="logContainerRef"
      class="flex-1 overflow-y-auto max-w-full h-full rounded-xl bg-slate-950 px-4 py-3 text-sm text-slate-200 shadow-card"
    >
      <pre class="m-0 whitespace-pre-wrap break-words font-mono leading-6"><code>
        <div
          v-for="(line, index) in logQueue"
          :key="index"
          class="log-line"
          v-html="highlightLine(line)"
        />
      </code></pre>
    </div>
  </div>
</template>

<style
  lang="scss"
  scoped
>
.log-line {
  min-height: 1.5rem;
}

:deep(.hljs-string) {
  color: #93c5fd;
}

:deep(.hljs-number) {
  color: #fcd34d;
}

:deep(pre),
:deep(code) {
  user-select: text;
  -webkit-user-select: text;
}
</style>
