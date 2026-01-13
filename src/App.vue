<script setup lang="ts">
import { useTheme } from '@/utils/theme'
import MenuView from '@/views/menu/MenuView.vue'

const { isDark } = useTheme()
</script>

<template>
  <div :class="{ 'dark': isDark }" class="flex flex-col w-full h-full bg-[#fdfdfd] dark:bg-[#373839]">
    <div class="flex w-full h-full">
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
