<script setup lang="ts">
import { getCurrentInstance, onMounted } from 'vue'
import { NConfigProvider, NMessageProvider, useMessage } from 'naive-ui'
import { useTheme } from '@/utils/theme'
import MenuView from '@/views/menu/MenuView.vue'
import 'vfonts/FiraCode.css'
import 'vfonts/Lato.css'

const { theme, lightThemeOverrides, darkThemeOverrides } = useTheme()

onMounted(() => {
  const vueInstance = getCurrentInstance()
  vueInstance!.appContext.config.globalProperties.$message = useMessage()
})
</script>

<template>
  <n-config-provider
    :theme="theme"
    :theme-overrides="theme === null ? lightThemeOverrides : darkThemeOverrides"
    class="flex w-full h-full bg-[#fdfdfd] dark:bg-[#373839]"
  >
    <n-message-provider>
      <div class="w-48">
        <menu-view />
      </div>
      <div class="flex-1 p-4 h-full w-full">
        <router-view />
      </div>
    </n-message-provider>
  </n-config-provider>
</template>
