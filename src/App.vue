<script setup lang="ts">
import { NConfigProvider } from 'naive-ui'
import { useTheme } from '@/utils/theme'
import MenuView from '@/views/menu/Index.vue'

const { theme, themeOverrides } = useTheme()
</script>

<template>
  <n-config-provider
    :theme="theme"
    :theme-overrides="themeOverrides"
    class="flex w-full h-full bg-[#fdfdfd] dark:bg-black"
  >
    <div class="w-48">
      <menu-view />
    </div>
    <div class="flex-1 p-4 h-full w-full">
      <router-view />
    </div>
  </n-config-provider>
</template>
