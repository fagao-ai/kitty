<script
  setup
  lang="ts"
>
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
