<script setup lang="ts">
import { onBeforeUnmount, watch } from 'vue'
import { watchOnce } from '@vueuse/core'
import PrimeInputNumber from 'primevue/inputnumber'
import PrimeInputText from 'primevue/inputtext'
import PrimeSkeleton from 'primevue/skeleton'
import PrimeToggleSwitch from 'primevue/toggleswitch'
import { isEnabled } from '@tauri-apps/plugin-autostart'
import { useI18n } from 'vue-i18n'
import HeaderBar from '@/components/HeaderBar.vue'
import { settingStore } from '@/views/setting/store'
import { useConfig } from '@/views/setting/hook'

defineEmits<{
  toggleMobileMenu: []
}>()

const { t, locale } = useI18n()
const { baseConfig, handleSwitchAutoStart, handleBaseConfigUpdate, handleSwitchProxy, handleLogLevelChange, loading, proxyLoading, initConfig } = useConfig()
initConfig()

const languageOptions = ['zh-CN', 'en-US']
const logLevelOptions = ['debug', 'info', 'warn', 'error']

async function handleLanguageChange(lang: string) {
  locale.value = lang
  handleBaseConfigUpdate()
}

async function handleAutoStart(val: boolean) {
  await handleSwitchAutoStart(val)
  handleBaseConfigUpdate()
}

async function handleUpdateInterval() {
  settingStore.value.autoUpdate = baseConfig.updateInterval
  await handleBaseConfigUpdate()
}

async function handleProxy(val: boolean) {
  await handleSwitchProxy(val)
  handleBaseConfigUpdate()
}

watchOnce(() => baseConfig.autoStart, async () => {
  baseConfig.autoStart = await isEnabled()
  // Only update if id is valid (loaded from backend)
  if (baseConfig.id > 0) {
    await handleBaseConfigUpdate()
  }
}, { immediate: true })

const unwatchProxyEnable = watch(() => baseConfig.sysproxyFlag, () => {
  settingStore.value.sysproxyFlag = baseConfig.sysproxyFlag
}, { immediate: true })

const unwatchProxyPort = watch(() => baseConfig.httpPort, () => {
  settingStore.value.port = baseConfig.httpPort
}, { immediate: true })

onBeforeUnmount(() => {
  unwatchProxyEnable()
  unwatchProxyPort()
})
</script>

<template>
  <div class="w-full h-full flex flex-col gap-y-4">
    <header-bar @toggle-mobile-menu="$emit('toggleMobileMenu')">
      <template #mobile-menu-button>
        <span class="inline-flex h-6 w-6 items-center justify-center">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M3 12h18M3 6h18M3 18h18" stroke-linecap="round" stroke-linejoin="round" />
          </svg>
        </span>
      </template>
      <template #title>
        {{ t('setting.title') }}
      </template>
    </header-bar>
    <div class="flex-1 flex flex-col gap-y-6 pr-6">
      <div
        class="grid grid-cols-2 grid-rows-2 gap-x-16 gap-y-4 p-6 bg-bg-card dark:bg-dark-bg-card shadow-card rounded-lg text-text-secondary dark:text-text-secondary"
      >
        <prime-skeleton
          v-if="loading"
          width="100%"
          height="34px"
          border-radius="999px"
        />
        <div
          v-else
          class="flex justify-between items-center"
        >
          <div class="font-semibold text-text-primary dark:text-text-primary">
            {{ t('setting.autoStart') }}
          </div>
          <div class="font-medium">
            <prime-toggle-switch
              v-model="baseConfig.autoStart"
              @update:model-value="handleAutoStart"
            />
          </div>
        </div>
        <prime-skeleton
          v-if="loading"
          width="100%"
          height="34px"
          border-radius="999px"
        />
        <div
          v-else
          class="flex justify-between items-center"
        >
          <div class="font-semibold text-text-primary dark:text-text-primary">
            {{ t('setting.language') }}
          </div>
          <div class="font-medium flex rounded-xl bg-slate-100 p-1 dark:bg-slate-800">
            <button
              v-for="lang in languageOptions"
              :key="lang"
              type="button"
              class="settings-pill"
              :class="{ 'settings-pill-active': baseConfig.language === lang }"
              @click="handleLanguageChange(lang)"
            >
              {{ lang === 'zh-CN' ? '文' : 'En' }}
            </button>
          </div>
        </div>
        <prime-skeleton
          v-if="loading"
          width="100%"
          height="34px"
          border-radius="999px"
        />
        <div
          v-else
          class="flex justify-between items-center"
        >
          <div class="font-semibold text-text-primary dark:text-text-primary">
            {{ t('setting.systemProxy') }}
          </div>
          <div class="font-medium">
            <prime-toggle-switch
              v-model="baseConfig.sysproxyFlag"
              :disabled="proxyLoading"
              @update:model-value="handleProxy"
            />
          </div>
        </div>
        <prime-skeleton
          v-if="loading"
          width="100%"
          height="34px"
          border-radius="999px"
        />
        <div
          v-else
          class="flex justify-between items-center"
        >
          <div class="font-semibold text-text-primary dark:text-text-primary">
            {{ t('setting.allowLan') }}
          </div>
          <div class="font-medium text-text-muted dark:text-text-muted">
            Off
          </div>
        </div>
      </div>
      <div
        class="grid grid-cols-2 grid-rows-2 gap-x-16 gap-y-4 p-6 bg-bg-card dark:bg-dark-bg-card shadow-card rounded-lg text-text-secondary dark:text-text-secondary"
      >
        <prime-skeleton
          v-if="loading"
          width="100%"
          height="34px"
          border-radius="999px"
        />
        <div
          v-else
          class="flex justify-between items-center"
        >
          <div class="font-semibold text-text-primary dark:text-text-primary">
            {{ t('setting.mode') }}
          </div>
          <div class="font-medium">
            {{ t('common.rules') }}
          </div>
        </div>
        <prime-skeleton
          v-if="loading"
          width="100%"
          height="34px"
          border-radius="999px"
        />
        <div
          v-else
          class="flex justify-between items-center"
        >
          <div class="font-semibold text-text-primary dark:text-text-primary">
            {{ t('setting.socks5Port') }}
          </div>
          <div class="font-medium w-20">
            <prime-input-number
              v-model="baseConfig.socksPort"
              :max="65535"
              :min="1"
              input-class="w-full"
              @blur="handleBaseConfigUpdate"
            />
          </div>
        </div>
        <prime-skeleton
          v-if="loading"
          width="100%"
          height="34px"
          border-radius="999px"
        />
        <div
          v-else
          class="flex justify-between items-center"
        >
          <div class="font-semibold text-text-primary dark:text-text-primary">
            {{ t('setting.httpPort') }}
          </div>
          <div class="font-medium w-20">
            <prime-input-number
              v-model="baseConfig.httpPort"
              :max="65535"
              :min="1"
              input-class="w-full"
              @blur="handleBaseConfigUpdate"
            />
          </div>
        </div>
        <prime-skeleton
          v-if="loading"
          width="100%"
          height="34px"
          border-radius="999px"
        />
        <div
          v-else
          class="flex justify-between items-center"
        >
          <div class="font-semibold text-text-primary dark:text-text-primary">
            {{ t('setting.delayTestUrl') }}
          </div>
          <div class="font-medium w-60">
            <prime-input-text
              v-model="baseConfig.delayTestUrl"
              class="w-full"
              @blur="handleBaseConfigUpdate"
            />
          </div>
        </div>
        <prime-skeleton
          v-if="loading"
          width="100%"
          height="34px"
          border-radius="999px"
        />
        <div
          v-else
          class="flex justify-between items-center"
        >
          <div class="font-semibold text-text-primary dark:text-text-primary">
            {{ t('setting.subscriptionAutoUpdate') }}
          </div>
          <div class="font-medium w-20">
            <prime-input-number
              v-model="baseConfig.updateInterval"
              :max="48"
              :min="1"
              input-class="w-full"
              @blur="handleUpdateInterval"
            />
          </div>
        </div>
      </div>
      <div
        class="grid grid-cols-2 grid-rows-2 gap-x-16 gap-y-4 p-6 bg-bg-card dark:bg-dark-bg-card shadow-card rounded-lg text-text-secondary dark:text-text-secondary"
      >
        <prime-skeleton
          v-if="loading"
          width="100%"
          height="34px"
          border-radius="999px"
        />
        <div
          v-else
          class="flex justify-between items-center col-span-2"
        >
          <div class="font-semibold text-text-primary dark:text-text-primary">
            {{ t('setting.logLevel') }}
          </div>
          <div class="font-medium flex rounded-xl bg-slate-100 p-1 dark:bg-slate-800">
            <button
              v-for="level in logLevelOptions"
              :key="level"
              type="button"
              class="settings-pill"
              :class="{ 'settings-pill-active': baseConfig.logLevel === level }"
              @click="handleLogLevelChange(level)"
            >
              {{ level.charAt(0).toUpperCase() + level.slice(1) }}
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
.settings-pill {
  @apply rounded-lg px-3 py-2 text-sm font-medium transition-colors;
  @apply text-text-secondary dark:text-text-secondary;
}

.settings-pill-active {
  @apply bg-white text-text-primary shadow-sm dark:bg-slate-700 dark:text-white;
}
</style>
