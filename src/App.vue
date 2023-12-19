<script setup lang="ts">
import { NConfigProvider } from 'naive-ui'
import { useTheme } from '@/utils/theme'
import MenuView from '@/views/menu/MenuView.vue'
import 'vfonts/Lato.css'
import 'vfonts/FiraCode.css'

const { theme, lightThemeOverrides, darkThemeOverrides } = useTheme()
</script>

<template>
  <n-config-provider
    :theme="theme"
    :theme-overrides="theme === null ? lightThemeOverrides : darkThemeOverrides"
    class="flex w-full h-full bg-[#fdfdfd] dark:bg-[#373839]"
  >
    <div class="w-48">
      <menu-view />
    </div>
    <div class="flex-1 p-4 h-full w-full">
      <router-view />
    </div>
  </n-config-provider>
</template>
