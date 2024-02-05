<script setup lang="ts">
import { NConfigProvider, NMessageProvider } from 'naive-ui'
import hljs from 'highlight.js/lib/core'
import { useTheme } from '@/utils/theme'
import MenuView from '@/views/menu/MenuView.vue'
import 'vfonts/FiraCode.css'
import 'vfonts/Lato.css'

const { theme, lightThemeOverrides, darkThemeOverrides } = useTheme()

hljs.registerLanguage('naive-log', () => ({
  contains: [
    {
      className: 'number',
      begin: /\d+/,
    },
  ],
}))
</script>

<template>
  <n-config-provider
    :theme="theme"
    :theme-overrides="theme === null ? lightThemeOverrides : darkThemeOverrides"
    :hljs="hljs"
    class="flex w-full h-full bg-[#fdfdfd] dark:bg-[#373839]"
  >
    <n-message-provider>
      <div class="w-48">
        <menu-view />
      </div>
      <div class="flex-1 p-4 h-full w-full overflow-y-hidden">
        <router-view />
      </div>
    </n-message-provider>
  </n-config-provider>
</template>
