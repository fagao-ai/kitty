<script setup lang="ts">
import { reactive, ref, unref, watch } from 'vue'
import { NRadioGroup, NSwitch } from 'naive-ui'
import { decamelizeKeys } from 'humps'
import { useI18n } from 'vue-i18n'
import { invoke } from '@/utils/invoke'
import { settingStore } from '@/views/setting/store'
import type { KittyBaseConfig } from '@/types/setting'

const { t, locale } = useI18n()

const proxyStatus = ref(false)
const proxyLoading = ref(false)
async function handleSwitchProxy(value: boolean) {
  proxyLoading.value = true
  try {
    if (value) {
      const res = await invoke<boolean>('get_hysteria_status')
      if (res.data)
        return

      await invoke('start_hysteria')
    }
    else { await invoke('stop_hysteria') }
  }
  finally {
    proxyLoading.value = false
  }
}

const baseConfig = reactive<KittyBaseConfig>({
  id: 0,
  httpPort: 10086,
  socksPort: 10087,
})

async function getBaseConfig() {
  const config = await invoke<KittyBaseConfig>('query_base_config')
  Object.assign(baseConfig, config.data)
}

async function getHysteriaStatus() {
  const res = await invoke<boolean>('get_hysteria_status')
  proxyStatus.value = res.data
}

async function onBaseConfigUpdate() {
  await invoke('update_base_config', { id: baseConfig.id, base_config: decamelizeKeys(baseConfig) })
}
getBaseConfig()
getHysteriaStatus()

const language = ref(unref(settingStore).language)

function whenLanguageChanged(lang: string) {
  settingStore.value.language = lang
  // language.value = lang
  locale.value = lang
}

watch(language, whenLanguageChanged, { immediate: true })
</script>

<template>
  <div class="w-full h-full flex flex-col space-y-4">
    <div class="h-8 text-2xl text-primay font-extrabold">
      {{ t('setting.title') }}
    </div>
    <div class="flex-1 flex flex-col space-y-6">
      <div
        class="dark:bg-dark grid grid-cols-2 grid-rows-2 gap-x-16 gap-y-4 p-6 bg-[#f9f7f7] shadow-lg rounded-md text-[#5b7497] dark:text-slate-300"
      >
        <div class="flex justify-between items-center">
          <div class="font-semibold">
            {{ t('setting.autoStart') }}
          </div>
          <div class="font-medium">
            <n-switch
              :value="false"
              :disabled="true"
              size="medium"
            />
          </div>
        </div>
        <div class="flex justify-between items-center">
          <div class="font-semibold">
            {{ t('setting.language') }}
          </div>
          <div class="font-medium">
            <n-radio-group
              v-model:value="language"
              name="langGroup"
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
        <div class="flex justify-between items-center">
          <div class="font-semibold">
            {{ t('setting.systemProxy') }}
          </div>
          <div class="font-medium">
            <n-switch
              v-model:value="proxyStatus"
              :loading="proxyLoading"
              size="medium"
              @update:value="handleSwitchProxy"
            />
          </div>
        </div>
        <div class="flex justify-between items-center">
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
        <div class="flex justify-between items-center">
          <div class="font-semibold">
            {{ t('setting.mode') }}
          </div>
          <div class="font-medium">
            {{ t('common.global') }}
          </div>
        </div>
        <div class="flex justify-between items-center">
          <div class="font-semibold">
            {{ t('setting.socks5Port') }}
          </div>
          <div class="font-medium w-20">
            <n-input-number
              v-model:value="baseConfig.socksPort"
              type="text"
              :show-button="false"
              :max="65535"
              :min="1"
              @blur="onBaseConfigUpdate"
            />
          </div>
        </div>
        <div class="flex justify-between items-center">
          <div class="font-semibold">
            {{ t('setting.httpPort') }}
          </div>
          <div class="font-medium w-20">
            <n-input-number
              v-model:value="baseConfig.httpPort"
              type="text"
              :show-button="false"
              :max="65535"
              :min="1"
              @blur="onBaseConfigUpdate"
            />
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
