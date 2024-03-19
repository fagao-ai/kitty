<script
  setup
  lang="ts"
>
import { h } from 'vue'
import { NMenu, useMessage } from 'naive-ui'
import type { MenuOption } from 'naive-ui'
import { RouterLink, useRoute } from 'vue-router'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

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
        class="text-5xl font-bold h-1/6 flex flex-center text-primay cursor-default"
      >
        kitty
      </div>
      <div
        class="flex-1 text-white text-lg"
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
      <div class="text-primay text-xs">
        {{ t('menubar.version') }}
      </div>
      <div class="text-primay text-xs">
        0.0.1.beta
      </div>
    </div>
  </div>
</template>

<style
  scoped
  lang="scss"
>
:deep(.n-menu) {
  .n-menu-item-content {
    @apply flex justify-center items-center;
    padding-left: 0 !important;
    padding-right: 0 !important;

    .n-menu-item-content-header {

      .router-link-active {
        font-size: 18px;
      }
    }
  }
}
</style>
