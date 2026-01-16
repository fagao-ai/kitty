<script setup lang="ts">
import { useTheme } from '@/utils/theme'
import MenuView from '@/views/menu/MenuView.vue'

const { isDark } = useTheme()
</script>

<template>
  <div
    :class="{ dark: isDark }"
    class="w-full h-full relative"
  >
    <!-- Animated Gradient Background with Floating Orbs -->
    <div class="fixed inset-0 -z-10 overflow-hidden">
      <!-- Base gradient background -->
      <div class="gradient-bg absolute inset-0" />

      <!-- Floating orbs for depth -->
      <div class="floating-orb orb-1 absolute w-96 h-96 bg-purple-500/30 dark:bg-purple-500/20 rounded-full blur-3xl -top-48 -left-48" />
      <div
        class="floating-orb orb-2 absolute w-80 h-80 bg-pink-500/20 dark:bg-pink-500/15 rounded-full blur-3xl -bottom-40 -right-40"
        style="animation-delay: -5s;"
      />
      <div
        class="floating-orb orb-3 absolute w-64 h-64 bg-blue-500/20 dark:bg-blue-500/15 rounded-full blur-3xl top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2"
        style="animation-delay: -10s;"
      />
    </div>

    <!-- Main Layout -->
    <div class="flex w-full h-full! relative z-10">
      <div
        class="w-48"
        data-tauri-drag-region
      >
        <MenuView />
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
