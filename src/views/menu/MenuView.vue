<script setup lang="ts">
import { h } from 'vue'
import { NMenu, useMessage } from 'naive-ui'
import type { MenuOption } from 'naive-ui'
import { RouterLink, useRoute } from 'vue-router'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

// @ts-expect-error @ts-expect-error
const version = __APP_VERSION__ as string

const menuOptions: MenuOption[] = [
  {
    label: () =>
      h(
        RouterLink,
        {
          to: {
            name: 'proxy',
          },
        },
        { default: () => t('menubar.proxies') },
      ),
    key: 'proxy',
  },
  {
    label: () =>
      h(
        RouterLink,
        {
          to: {
            name: 'subscription',
          },
        },
        { default: () => t('menubar.subscriptions') },
      ),
    key: 'subscription',
  },
  {
    label: () =>
      h(
        RouterLink,
        {
          to: {
            name: 'rule',
          },
        },
        { default: () => t('menubar.rules') },
      ),
    key: 'rule',
  },
  {
    label: () =>
      h(
        RouterLink,
        {
          to: {
            name: 'log',
          },
        },
        { default: () => t('menubar.logs') },
      ),
    key: 'log',
  },
  {
    label: () =>
      h(
        RouterLink,
        {
          to: {
            name: 'setting',
          },
        },
        { default: () => t('menubar.settings') },
      ),
    key: 'setting',
  },
]

const route = useRoute()
window.$message = useMessage()
</script>

<template>
  <div
    class="flex flex-col h-full px-4 pb-4"
    data-tauri-drag-region
  >
    <div class="flex-1 flex flex-col">
      <div
        data-tauri-drag-region
        class="text-5xl font-bold h-1/6 flex flex-center text-primary cursor-default tracking-tight"
      >
        kitty
      </div>
      <div
        class="flex-1 text-lg"
        data-tauri-drag-region
      >
        <n-menu
          :value="route.name as string ?? 'proxy'"
          :default-value="route.name as string ?? 'proxy'"
          :options="menuOptions"
        />
      </div>
    </div>
    <div class="h-1/8 flex flex-center flex-col">
      <div class="text-text-secondary dark:text-text-muted text-xs font-medium">
        {{ t('menubar.version') }}
      </div>
      <div class="text-text-muted dark:text-text-muted text-xs">
        {{ `${version}.beta` }}
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
:deep(.n-menu) {
  .n-menu-item-content {
    @apply flex justify-center items-center;
    padding-left: 0 !important;
    padding-right: 0 !important;
    margin-bottom: 4px;
    border-radius: 8px;
    transition: all 0.2s ease;

    .n-menu-item-content-header {
      .router-link-active {
        font-weight: 500;
      }
    }

    &:hover {
      @apply bg-bg-muted dark:bg-dark-bg-muted;
    }
  }
}
</style>
