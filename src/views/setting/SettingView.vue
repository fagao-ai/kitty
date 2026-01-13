<script setup lang="ts">
import { onBeforeUnmount, watch } from 'vue'
import { watchOnce } from '@vueuse/core'
import { useI18n } from 'vue-i18n'
import Skeleton from 'primevue/skeleton'
import ToggleSwitch from 'primevue/toggleswitch'
import InputNumber from 'primevue/inputnumber'
import InputText from 'primevue/inputtext'
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
  await handleBaseConfigUpdate()
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
    <header-bar>
      <template #title>
        {{ t('setting.title') }}
      </template>
    </header-bar>
    <div class="flex-1 flex flex-col gap-y-6 pr-4">
      <div
        class="dark:bg-dark grid grid-cols-2 grid-rows-2 gap-x-16 gap-y-4 p-6 bg-[#f9f7f7] shadow-lg rounded-md text-[#5b7497] dark:text-slate-300"
      >
        <Skeleton
          v-if="loading"
          width="100%"
          height="2.5rem"
          border-radius="12px"
        />
        <div
          v-else
          class="flex justify-between items-center"
        >
          <div class="font-semibold">
            {{ t('setting.autoStart') }}
          </div>
          <div class="font-medium">
            <ToggleSwitch
              v-model="baseConfig.autoStart"
              @update:model-value="handleAutoStart"
            />
          </div>
        </div>
        <Skeleton
          v-if="loading"
          width="100%"
          height="2.5rem"
          border-radius="12px"
        />
        <div
          v-else
          class="flex justify-between items-center"
        >
          <div class="font-semibold">
            {{ t('setting.language') }}
          </div>
          <div class="font-medium">
            <div class="flex gap-2 bg-gray-100 dark:bg-gray-800 rounded-full p-1">
              <input
                :id="'lang-zh'"
                v-model="baseConfig.language"
                type="radio"
                name="langGroup"
                value="zh-CN"
                class="hidden"
                @change="handleLanguageChange('zh-CN')"
              >
              <label
                for="lang-zh"
                class="px-4 py-1 rounded-full cursor-pointer text-sm transition-all"
                :class="{ 'bg-emerald-500 text-white': baseConfig.language === 'zh-CN', 'hover:bg-gray-200 dark:hover:bg-gray-700': baseConfig.language !== 'zh-CN' }"
              >
                文
              </label>

              <input
                :id="'lang-en'"
                v-model="baseConfig.language"
                type="radio"
                name="langGroup"
                value="en-US"
                class="hidden"
                @change="handleLanguageChange('en-US')"
              >
              <label
                for="lang-en"
                class="px-4 py-1 rounded-full cursor-pointer text-sm transition-all"
                :class="{ 'bg-emerald-500 text-white': baseConfig.language === 'en-US', 'hover:bg-gray-200 dark:hover:bg-gray-700': baseConfig.language !== 'en-US' }"
              >
                En
              </label>
            </div>
          </div>
        </div>
        <Skeleton
          v-if="loading"
          width="100%"
          height="2.5rem"
          border-radius="12px"
        />
        <div
          v-else
          class="flex justify-between items-center"
        >
          <div class="font-semibold">
            {{ t('setting.systemProxy') }}
          </div>
          <div class="font-medium">
            <ToggleSwitch
              v-model="baseConfig.sysproxyFlag"
              @update:model-value="handleProxy"
            />
          </div>
        </div>
        <Skeleton
          v-if="loading"
          width="100%"
          height="2.5rem"
          border-radius="12px"
        />
        <div
          v-else
          class="flex justify-between items-center"
        >
          <div class="font-semibold">
            {{ t('setting.allowLan') }}
          </div>
          <div class="font-medium">
            Off
          </div>
        </div>
      </div>
      <div
        class="dark:bg-dark dark:text-slate-300 grid grid-cols-2 grid-rows-2 gap-x-16 gap-y-4 p-6 text-[#5b7497] bg-[#f9f7f7] shadow-lg rounded-md"
      >
        <Skeleton
          v-if="loading"
          width="100%"
          height="2.5rem"
          border-radius="12px"
        />
        <div
          v-else
          class="flex justify-between items-center"
        >
          <div class="font-semibold">
            {{ t('setting.mode') }}
          </div>
          <div class="font-medium">
            {{ t('common.rules') }}
          </div>
        </div>
        <Skeleton
          v-if="loading"
          width="100%"
          height="2.5rem"
          border-radius="12px"
        />
        <div
          v-else
          class="flex justify-between items-center"
        >
          <div class="font-semibold">
            {{ t('setting.socks5Port') }}
          </div>
          <div class="font-medium w-24">
            <InputNumber
              v-model="baseConfig.socksPort"
              :max="65535"
              :min="1"
              show-buttons
              :use-grouping="false"
              @blur="handleBaseConfigUpdate"
            />
          </div>
        </div>
        <Skeleton
          v-if="loading"
          width="100%"
          height="2.5rem"
          border-radius="12px"
        />
        <div
          v-else
          class="flex justify-between items-center"
        >
          <div class="font-semibold">
            {{ t('setting.httpPort') }}
          </div>
          <div class="font-medium w-24">
            <InputNumber
              v-model="baseConfig.httpPort"
              :max="65535"
              :min="1"
              show-buttons
              :use-grouping="false"
              @blur="handleBaseConfigUpdate"
            />
          </div>
        </div>
        <Skeleton
          v-if="loading"
          width="100%"
          height="2.5rem"
          border-radius="12px"
        />
        <div
          v-else
          class="flex justify-between items-center"
        >
          <div class="font-semibold">
            {{ t('setting.delayTestUrl') }}
          </div>
          <div class="font-medium w-60">
            <InputText
              v-model="baseConfig.delayTestUrl"
              type="text"
              class="w-full"
              @blur="handleBaseConfigUpdate"
            />
          </div>
        </div>
        <Skeleton
          v-if="loading"
          width="100%"
          height="2.5rem"
          border-radius="12px"
        />
        <div
          v-else
          class="flex justify-between items-center"
        >
          <div class="font-semibold">
            {{ t('setting.subscriptionAutoUpdate') }}
          </div>
          <div class="font-medium w-24">
            <InputNumber
              v-model="baseConfig.updateInterval"
              :max="48"
              :min="1"
              show-buttons
              :use-grouping="false"
              @blur="handleUpdateInterval"
            />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
