<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { useToast } from 'primevue/usetoast'
import Menu from 'primevue/menu'

const { t } = useI18n()
const router = useRouter()
const toast = useToast()

// @ts-expect-error @ts-expect-error
const version = __APP_VERSION__ as string

const menuItems = ref([
  {
    label: t('menubar.proxies'),
    icon: 'pi pi-home',
    command: () => router.push({ name: 'proxy' }),
  },
  {
    label: t('menubar.rules'),
    icon: 'pi pi-list',
    command: () => router.push({ name: 'rule' }),
  },
  {
    label: t('menubar.logs'),
    icon: 'pi pi-file',
    command: () => router.push({ name: 'log' }),
  },
  {
    label: t('menubar.settings'),
    icon: 'pi pi-cog',
    command: () => router.push({ name: 'setting' }),
  },
])

// 全局消息服务
window.$message = toast
</script>

<template>
  <div
    class="flex flex-col h-full px-4 pb-4"
    data-tauri-drag-region
  >
    <div class="flex-1 flex flex-col">
      <div
        data-tauri-drag-region
        class="text-5xl font-bold h-1/6 flex flex-center text-primay cursor-default"
      >
        kitty
      </div>
      <div
        class="flex-1 text-white text-lg"
        data-tauri-drag-region
      >
        <Menu :model="menuItems" class="w-full">
          <template #item="{ item, props }">
            <a v-ripple class="flex items-center" v-bind="props.action" @click="item.command">
              <span :class="item.icon" />
              <span class="ml-2">{{ item.label }}</span>
            </a>
          </template>
        </Menu>
      </div>
    </div>
    <div class="h-1/8 flex flex-center flex-col">
      <div class="text-primay text-xs">
        {{ t('menubar.version') }}
      </div>
      <div class="text-primay text-xs">
        {{ `${version}.beta` }}
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
:deep(.p-menu) {
  .p-menuitem-link {
    @apply flex justify-center items-center;
    padding-left: 0 !important;
    padding-right: 0 !important;

    .router-link-active {
      font-size: 18px;
    }
  }
}
</style>
