<script setup lang="ts">
import { ref } from 'vue'
import { NConfigProvider, NMessageProvider, NDrawer, NDrawerContent } from 'naive-ui'
import hljs from 'highlight.js/lib/core'
import { useTheme } from '@/utils/theme'
import MenuView from '@/views/menu/MenuView.vue'
import 'vfonts/FiraCode.css'
import 'vfonts/Lato.css'

const { theme, lightThemeOverrides, darkThemeOverrides } = useTheme()
const mobileMenuOpen = ref(false)

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
    class="flex flex-col w-full h-full bg-bg dark:bg-dark-bg-muted"
  >
    <div class="flex w-full h-full">
      <n-message-provider>
        <!-- Desktop sidebar -->
        <div
          class="hidden md:block w-48 lg:w-52 xl:w-56 2xl:w-60 shrink-0 bg-bg-muted dark:bg-dark-bg-muted"
          data-tauri-drag-region
        >
          <menu-view />
        </div>

        <!-- Mobile drawer menu -->
        <n-drawer
          v-model:show="mobileMenuOpen"
          :width="280"
          placement="left"
          class="md:hidden"
        >
          <n-drawer-content>
            <menu-view @menu-item-click="mobileMenuOpen = false" />
          </n-drawer-content>
        </n-drawer>

        <!-- Main content -->
        <div
          class="flex-1 h-full w-full overflow-y-hidden bg-bg dark:bg-dark-bg"
          data-tauri-drag-region
        >
          <router-view v-slot="{ Component }">
            <transition name="fade">
              <keep-alive>
                <component :is="Component" :is-mobile-menu-open="mobileMenuOpen" @toggle-mobile-menu="mobileMenuOpen = !mobileMenuOpen" />
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
