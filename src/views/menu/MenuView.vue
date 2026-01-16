<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { useToast } from 'primevue/usetoast'
import Menu from 'primevue/menu'
import type { MenuItemCommandEvent } from 'primevue/menuitem'

const { t } = useI18n()
const router = useRouter()
const toast = useToast()

// @ts-expect-error @ts-expect-error
const version = __APP_VERSION__ as string

const menuItems = ref([
  {
    label: t('menubar.proxies'),
    icon: 'pi pi-home',
    command: (_event: MenuItemCommandEvent) => router.push({ name: 'proxy' }),
  },
  {
    label: t('menubar.rules'),
    icon: 'pi pi-list',
    command: (_event: MenuItemCommandEvent) => router.push({ name: 'rule' }),
  },
  {
    label: t('menubar.logs'),
    icon: 'pi pi-file',
    command: (_event: MenuItemCommandEvent) => router.push({ name: 'log' }),
  },
  {
    label: t('menubar.settings'),
    icon: 'pi pi-cog',
    command: (_event: MenuItemCommandEvent) => router.push({ name: 'setting' }),
  },
])

// Global message service
window.$message = toast
</script>

<template>
  <div
    class="glass-sidebar flex flex-col h-full py-4"
    data-tauri-drag-region
  >
    <div class="flex-1 flex flex-col">
      <!-- Logo with gradient text -->
      <div
        data-tauri-drag-region
        class="text-5xl font-bold h-1/6 flex flex-center cursor-default"
      >
        <span class="text-gradient-purple">
          kitty
        </span>
      </div>

      <!-- Menu Items -->
      <div class="flex-1 w-full">
        <Menu
          class="min-w-48! h-full"
          :model="menuItems"
        />
      </div>
    </div>

    <!-- Version Info -->
    <div class="h-1/8 flex flex-center flex-col">
      <div class="text-purple-600 dark:text-purple-300 text-xs">
        {{ t('menubar.version') }}
      </div>
      <div class="text-purple-600 dark:text-purple-300 text-xs">
        {{ `${version}.beta` }}
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
.glass-sidebar {
  background: linear-gradient(180deg, rgba(139, 92, 246, 0.12) 0%, rgba(99, 102, 241, 0.06) 100%);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border-right: 1px solid rgba(255, 255, 255, 0.15);
}

.dark .glass-sidebar {
  background: linear-gradient(180deg, rgba(139, 92, 246, 0.15) 0%, rgba(99, 102, 241, 0.08) 100%);
  border-right: 1px solid rgba(255, 255, 255, 0.08);
}
</style>
