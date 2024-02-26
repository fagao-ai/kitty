<script setup lang="ts">
import { onUnmounted, watch } from 'vue'
import { NConfigProvider, NMessageProvider } from 'naive-ui'
import hljs from 'highlight.js/lib/core'
import { useTheme } from '@/utils/theme'
import { useSubscriptionAutoUpdate } from '@/tools'
import { settingStore } from '@/views/setting/store'
import MenuView from '@/views/menu/MenuView.vue'
import 'vfonts/FiraCode.css'
import 'vfonts/Lato.css'

const { theme, lightThemeOverrides, darkThemeOverrides } = useTheme()
const { stopAutoUpdate, autoUpdate } = useSubscriptionAutoUpdate()
hljs.registerLanguage('naive-log', () => ({
  contains: [
    {
      className: 'number',
      begin: /\d+/,
    },
  ],
}))

watch(settingStore, (val, oldVal) => {
  if (!oldVal || val.autoUpdate !== oldVal.autoUpdate) {
    stopAutoUpdate()
    autoUpdate(val.autoUpdate)
  }
}, { immediate: true })

onUnmounted(() => {
  stopAutoUpdate()
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
