<script setup lang="ts">
import { computed } from 'vue'
import { RouterLink as AppRouterLink, useRoute } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { useMessage } from '@/utils/message'

const emits = defineEmits<{
  menuItemClick: []
}>()
const { t } = useI18n()
// @ts-expect-error injected by Vite define
const version = __APP_VERSION__ as string

const menuOptions = computed(() => [
  { key: 'proxy', label: t('menubar.proxies') },
  { key: 'subscription', label: t('menubar.subscriptions') },
  { key: 'rule', label: t('menubar.rules') },
  { key: 'log', label: t('menubar.logs') },
  { key: 'setting', label: t('menubar.settings') },
])

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
        <nav class="flex flex-col gap-2">
          <app-router-link
            v-for="option in menuOptions"
            :key="option.key"
            :to="{ name: option.key }"
            class="menu-link"
            :class="{ 'menu-link-active': (route.name as string ?? 'proxy') === option.key }"
            @click="emits('menuItemClick')"
          >
            {{ option.label }}
          </app-router-link>
        </nav>
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
.menu-link {
  @apply flex items-center justify-center rounded-lg px-3 py-2.5 text-base font-medium;
  @apply text-text-secondary dark:text-text-secondary;
  @apply transition-all duration-200;

  &:hover {
    @apply bg-bg-card dark:bg-dark-bg;
    @apply text-text-primary dark:text-text-primary;
  }
}

.menu-link-active {
  @apply bg-primary text-white shadow-card;
}
</style>
