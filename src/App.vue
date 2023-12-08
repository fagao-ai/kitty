<script setup lang="ts">
import { NConfigProvider } from 'naive-ui'
import { useTheme } from '@/utils/theme'
import MenuView from '@/views/menu/MenuView.vue'

const { theme, themeOverrides } = useTheme()
</script>

<template>
  <NConfigProvider
    :theme="theme"
    :theme-overrides="themeOverrides"
    class="flex w-full h-full bg-[#fdfdfd] dark:bg-[#373839]"
  >
    <div class="w-48">
      <MenuView />
    </div>
    <div class="flex-1 p-4 h-full w-full">
      <router-view />
    </div>
  </NConfigProvider>
</template>
