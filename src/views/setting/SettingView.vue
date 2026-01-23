<script setup lang="ts">
import { onBeforeUnmount, watch } from 'vue'
import { watchOnce } from '@vueuse/core'
import { NRadioGroup, NSkeleton, NSwitch } from 'naive-ui'
import { isEnabled } from '@tauri-apps/plugin-autostart'
import { useI18n } from 'vue-i18n'
import HeaderBar from '@/components/HeaderBar.vue'
import { settingStore } from '@/views/setting/store'
import { useConfig } from '@/views/setting/hook'

const { t, locale } = useI18n()
const { baseConfig, handleSwitchAutoStart, handleBaseConfigUpdate, handleSwitchProxy, loading, proxyLoading, initConfig } = useConfig()
initConfig()

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
    <header-bar @toggle-mobile-menu="$emit('toggle-mobile-menu')">
      <template #mobile-menu-button>
        <n-icon size="24">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M3 12h18M3 6h18M3 18h18" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </n-icon>
      </template>
      <template #title>
        {{ t('setting.title') }}
      </template>
    </header-bar>
    <div class="flex-1 flex flex-col gap-y-6 pr-6">
      <div
        class="grid grid-cols-2 grid-rows-2 gap-x-16 gap-y-4 p-6 bg-bg-card dark:bg-dark-bg-card shadow-card rounded-lg text-text-secondary dark:text-text-secondary"
      >
        <n-skeleton
          v-if="loading"
          width="100%"
          :height="34"
          round
        />
        <div
          v-else
          class="flex justify-between items-center"
        >
          <div class="font-semibold text-text-primary dark:text-text-primary">
            {{ t('setting.autoStart') }}
          </div>
          <div class="font-medium">
            <n-switch
              v-model:value="baseConfig.autoStart"
              size="medium"
              @update:value="handleAutoStart"
            />
          </div>
        </div>
        <n-skeleton
          v-if="loading"
          width="100%"
          :height="34"
          round
        />
        <div
          v-else
          class="flex justify-between items-center"
        >
          <div class="font-semibold text-text-primary dark:text-text-primary">
            {{ t('setting.language') }}
          </div>
          <div class="font-medium">
            <n-radio-group
              v-model:value="baseConfig.language"
              name="langGroup"
              @update:value="handleLanguageChange"
            >
              <n-radio-button value="zh-CN">
                文
              </n-radio-button>
              <n-radio-button value="en-US">
                En
              </n-radio-button>
            </n-radio-group>
          </div>
        </div>
        <n-skeleton
          v-if="loading"
          width="100%"
          :height="34"
          round
        />
        <div
          v-else
          class="flex justify-between items-center"
        >
          <div class="font-semibold text-text-primary dark:text-text-primary">
            {{ t('setting.systemProxy') }}
          </div>
          <div class="font-medium">
            <n-switch
              v-model:value="baseConfig.sysproxyFlag"
              :loading="proxyLoading"
              size="medium"
              @update:value="handleProxy"
            />
          </div>
        </div>
        <n-skeleton
          v-if="loading"
          width="100%"
          :height="34"
          round
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
        <n-skeleton
          v-if="loading"
          width="100%"
          :height="34"
          round
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
        <n-skeleton
          v-if="loading"
          width="100%"
          :height="34"
          round
        />
        <div
          v-else
          class="flex justify-between items-center"
        >
          <div class="font-semibold text-text-primary dark:text-text-primary">
            {{ t('setting.socks5Port') }}
          </div>
          <div class="font-medium w-20">
            <n-input-number
              v-model:value="baseConfig.socksPort"
              type="text"
              :show-button="false"
              :max="65535"
              :min="1"
              @blur="handleBaseConfigUpdate"
            />
          </div>
        </div>
        <n-skeleton
          v-if="loading"
          width="100%"
          :height="34"
          round
        />
        <div
          v-else
          class="flex justify-between items-center"
        >
          <div class="font-semibold text-text-primary dark:text-text-primary">
            {{ t('setting.httpPort') }}
          </div>
          <div class="font-medium w-20">
            <n-input-number
              v-model:value="baseConfig.httpPort"
              type="text"
              :show-button="false"
              :max="65535"
              :min="1"
              @blur="handleBaseConfigUpdate"
            />
          </div>
        </div>
        <n-skeleton
          v-if="loading"
          width="100%"
          :height="34"
          round
        />
        <div
          v-else
          class="flex justify-between items-center"
        >
          <div class="font-semibold text-text-primary dark:text-text-primary">
            {{ t('setting.delayTestUrl') }}
          </div>
          <div class="font-medium w-60">
            <n-input
              v-model:value="baseConfig.delayTestUrl"
              type="text"
              @blur="handleBaseConfigUpdate"
            />
          </div>
        </div>
        <n-skeleton
          v-if="loading"
          width="100%"
          :height="34"
          round
        />
        <div
          v-else
          class="flex justify-between items-center"
        >
          <div class="font-semibold text-text-primary dark:text-text-primary">
            {{ t('setting.subscriptionAutoUpdate') }}
          </div>
          <div class="font-medium w-20">
            <n-input-number
              v-model:value="baseConfig.updateInterval"
              :show-button="false"
              :max="48"
              :min="1"
              @blur="handleUpdateInterval"
            />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
