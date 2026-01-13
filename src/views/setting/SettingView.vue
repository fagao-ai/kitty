<script setup lang="ts">
import { onBeforeUnmount, watch } from 'vue'
import { watchOnce } from '@vueuse/core'
import { useI18n } from 'vue-i18n'
import Skeleton from 'primevue/skeleton'
import ToggleSwitch from 'primevue/toggleswitch'
import InputNumber from 'primevue/inputnumber'
import InputText from 'primevue/inputtext'
import SelectButton from 'primevue/selectbutton'
import HeaderBar from '@/components/HeaderBar.vue'
import { settingStore } from '@/views/setting/store'
import { useConfig } from '@/views/setting/hook'

const { t, locale } = useI18n()
const { baseConfig, handleSwitchAutoStart, handleBaseConfigUpdate, handleSwitchProxy, loading, initConfig, isEnabled } = useConfig()
initConfig()

const languageOptions = [
  { label: '文', value: 'zh-CN' },
  { label: 'En', value: 'en-US' },
]

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
    <HeaderBar>
      <template #title>
        {{ t('setting.title') }}
      </template>
    </HeaderBar>
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
            <SelectButton
              v-model="baseConfig.language"
              :options="languageOptions"
              option-label="label"
              option-value="value"
              @update:model-value="handleLanguageChange"
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
