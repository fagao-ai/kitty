<script setup lang="ts">
import { NConfigProvider, NMessageProvider } from 'naive-ui'
import hljs from 'highlight.js/lib/core'
import { useTheme } from '@/utils/theme'
import MenuView from '@/views/menu/MenuView.vue'
import 'vfonts/FiraCode.css'
import 'vfonts/Lato.css'

const { theme, lightThemeOverrides, darkThemeOverrides } = useTheme()
hljs.registerLanguage('kitty-log', () => ({
  contains: [
    // {
    //   className: 'string',
    //   keywords: ['proxy', 'direct']
    // },
    {
      className: 'string',
      begin: /\[[A-Z]+\]/,
    },
    {
      className: 'number',
      match: /^(?:https?:\/\/)?(?:www\.)?([a-zA-Z0-9-]+(?:\.[a-zA-Z0-9-]+)*)(?::\d+)?(?:\/.*)?$/,
    },
  ],
}))
</script>

<template>
  <n-config-provider
    :theme="theme"
    :theme-overrides="theme?.name === 'light' ? lightThemeOverrides : darkThemeOverrides"
    :hljs="hljs"
    class="flex flex-col w-full h-full bg-[#fdfdfd] dark:bg-[#373839]"
  >
    <div class="flex w-full h-full">
      <n-message-provider>
        <div
          class="w-48"
          data-tauri-drag-region
        >
          <menu-view />
        </div>
        <div
          class="flex-1 h-full w-full overflow-y-hidden"
          data-tauri-drag-region
        >
          <router-view v-slot="{ Component }">
            <transition name="fade">
              <keep-alive>
                <component :is="Component" />
              </keep-alive>
            </transition>
          </router-view>
        </div>
      </n-message-provider>
    </div>
  </n-config-provider>
</template>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
