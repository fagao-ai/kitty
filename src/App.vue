<script setup lang="ts">
import { ref } from 'vue'
import PrimeConfirmPopup from 'primevue/confirmpopup'
import PrimeDrawer from 'primevue/drawer'
import PrimeToast from 'primevue/toast'
import hljs from 'highlight.js/lib/core'
import { useTheme } from '@/utils/theme'
import MenuView from '@/views/menu/MenuView.vue'

useTheme()
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
  <div class="flex flex-col w-full h-full bg-bg dark:bg-dark-bg-muted">
    <prime-toast position="top-right" />
    <prime-confirm-popup />
    <div class="flex w-full h-full">
      <div
        class="hidden md:block w-48 lg:w-52 xl:w-56 2xl:w-60 shrink-0 bg-bg-muted dark:bg-dark-bg-muted"
        data-tauri-drag-region
      >
        <menu-view />
      </div>

      <prime-drawer
        v-model:visible="mobileMenuOpen"
        position="left"
        class="md:!hidden !w-[280px]"
      >
        <menu-view @menu-item-click="mobileMenuOpen = false" />
      </prime-drawer>

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
    </div>
  </div>
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
