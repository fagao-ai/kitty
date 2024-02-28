<script setup lang="ts">
import { onMounted, onUnmounted, watch } from 'vue'
import { NConfigProvider, NMessageProvider } from 'naive-ui'
import hljs from 'highlight.js/lib/core'
import { type UnlistenFn, listen } from '@tauri-apps/api/event'
import { useTheme } from '@/utils/theme'
import { useSubscriptionAutoUpdate } from '@/tools'
import { settingStore } from '@/views/setting/store'
import { useLogQueue } from '@/views/log/store'
import MenuView from '@/views/menu/MenuView.vue'
import 'vfonts/FiraCode.css'
import 'vfonts/Lato.css'

const { theme, lightThemeOverrides, darkThemeOverrides } = useTheme()
const { stopAutoUpdate } = useSubscriptionAutoUpdate()
hljs.registerLanguage('naive-log', () => ({
  contains: [
    {
      className: 'number',
      begin: /\d+/,
    },
  ],
}))

const { enqueueLog } = useLogQueue(1000)
// let ii = 1
// const logId = setInterval(() => {
//   enqueueLog(`${ii}`)
//   ii++
// }, 1000)

let unlisten: UnlistenFn | undefined
onMounted(async () => {
  unlisten = await listen<string>('kitty_logger', (event) => {
    console.log(`log is ${event.payload}`)

    enqueueLog(event.payload)
  })
})

watch(settingStore, (val, oldVal) => {
  if (!oldVal || val.autoUpdate !== oldVal.autoUpdate)
    stopAutoUpdate()
  // autoUpdate(val.autoUpdate)
}, { immediate: true })

onUnmounted(() => {
  // clearInterval(logId)
  stopAutoUpdate()
  unlisten?.()
})
</script>

<template>
  <n-config-provider
    :theme="theme"
    :theme-overrides="theme === null ? lightThemeOverrides : darkThemeOverrides"
    :hljs="hljs"
    class="flex flex-col w-full h-full bg-[#fdfdfd] dark:bg-[#373839]"
  >
    <div
      class="h-4 w-full cursor-move"
      data-tauri-drag-region
    >
      &nbsp;
    </div>
    <div class="flex-l flex w-full h-full">
      <n-message-provider>
        <div class="w-48">
          <menu-view />
        </div>
        <div class="flex-1 px-4 pb-4 h-full w-full overflow-y-hidden">
          <router-view />
        </div>
      </n-message-provider>
    </div>
  </n-config-provider>
</template>
