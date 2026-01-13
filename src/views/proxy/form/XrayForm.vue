<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import InputText from 'primevue/inputtext'
import InputNumber from 'primevue/inputnumber'
import ToggleSwitch from 'primevue/toggleswitch'
import ScrollPanel from 'primevue/scrollpanel'
import Select from 'primevue/select'
import Button from 'primevue/button'
import { useVModel } from '@vueuse/core'
import type { XrayProxy } from '@/types/proxy'

const props = defineProps<Props>()

const { t } = useI18n()

interface Props {
  form: XrayProxy
}
const formState = useVModel(props, 'form')

const streamSettingOptions = [
  { label: 'WebSocket', value: 'ws' },
  { label: 'Tcp', value: 'tcp' },
  { label: 'http2', value: 'http2' },
  { label: 'grpc', value: 'grpc' },
  { label: 'kcp', value: 'kcp' },
]

function handleRemoveHttp2Host(index: number) {
  if (formState.value.streamSettings.network !== 'http2')
    return
  formState.value.streamSettings.http2Settings.host.splice(index, 1)
}

function handleAddHttp2Host() {
  if (formState.value.streamSettings.network !== 'http2')
    return
  formState.value.streamSettings.http2Settings.host.push('')
}
</script>

<template>
  <ScrollPanel style="height: 100%;">
    <div class="flex flex-col gap-4">
      <div class="flex flex-col gap-2">
        <label class="font-semibold text-sm">{{ t('proxy.xray.proxyName') }}</label>
        <InputText v-model="formState.name" />
      </div>

      <div class="flex flex-col gap-2">
        <label class="font-semibold text-sm">{{ t('proxy.xray.protocol') }}</label>
        <Select
          v-model="formState.protocol"
          :options="[
            { label: 'vless', value: 'vless' },
            { label: 'vmess', value: 'vmess' },
            { label: 'trojan', value: 'trojan' },
          ]"
        />
      </div>

      <div class="flex flex-col gap-2">
        <label class="font-semibold text-sm">uuid</label>
        <InputText
          v-model="formState.uuid"
          placeholder="xxxx-xxxx-xxxx-xxxx"
        />
      </div>

      <div class="flex flex-col gap-2">
        <label class="font-semibold text-sm">{{ t('proxy.xray.address') }}</label>
        <InputText
          v-model="formState.address"
          placeholder="www.example.com"
        />
      </div>

      <div class="flex flex-col gap-2">
        <label class="font-semibold text-sm">{{ t('proxy.xray.port') }}</label>
        <InputNumber
          v-model="formState.port"
          :max="65535"
          :min="1"
          class="w-full"
          :show-buttons="false"
        />
      </div>

      <div class="flex flex-col gap-2">
        <label class="font-semibold text-sm">{{ t('proxy.xray.network') }}</label>
        <Select
          v-model="formState.streamSettings.network"
          :options="streamSettingOptions"
        />
      </div>

      <div class="flex flex-col gap-2">
        <label class="font-semibold text-sm">{{ t('proxy.xray.streamSetting.security') }}</label>
        <Select
          v-model="formState.streamSettings.security"
          :options="[
            { label: 'none', value: 'none' },
            { label: 'tls', value: 'tls' },
            { label: 'reality', value: 'reality' },
          ]"
        />
      </div>

      <div class="flex items-center gap-2">
        <ToggleSwitch v-model="formState.streamSettings.tlsSettings!.allowInsecure" />
        <label class="font-semibold text-sm cursor-pointer">{{ t('proxy.xray.streamSetting.tlsSettings.allowInsecure') }}</label>
      </div>

      <div class="flex flex-col gap-2">
        <label class="font-semibold text-sm">{{ t('proxy.xray.streamSetting.tlsSettings.serverName') }}</label>
        <InputText
          v-model="formState.streamSettings.tlsSettings!.serverName"
          placeholder="www.example.com"
        />
      </div>

      <template v-if="formState.streamSettings.network === 'ws'">
        <div class="flex flex-col gap-2">
          <label class="font-semibold text-sm">{{ t('proxy.streamSetting.wsSettings.path') }}</label>
          <InputText v-model="formState.streamSettings.wsSettings.path" />
        </div>

        <div class="flex flex-col gap-2">
          <label class="font-semibold text-sm">{{ t('proxy.streamSetting.wsSettings.host') }}</label>
          <InputText v-model="formState.streamSettings.wsSettings.headers.host" />
        </div>
      </template>

      <template v-if="formState.streamSettings.network === 'http2'">
        <div class="flex flex-col gap-2">
          <label class="font-semibold text-sm">{{ t('proxy.streamSetting.http2Settings.path') }}</label>
          <InputText v-model="formState.streamSettings.http2Settings.path" />
        </div>

        <div
          v-for="(item, index) in formState.streamSettings.http2Settings.host"
          :key="index"
          class="flex flex-col gap-2"
        >
          <label class="font-semibold text-sm">{{ `${t('proxy.xray.streamSetting.http2Settings.headers.host')}${index + 1}` }}</label>
          <div class="flex gap-2">
            <InputText v-model="formState.streamSettings.http2Settings.host[index]" class="flex-1" />
            <Button
              icon="pi pi-plus"
              @click="handleAddHttp2Host()"
            />
            <Button
              icon="pi pi-minus"
              severity="danger"
              @click="handleRemoveHttp2Host(index)"
            />
          </div>
        </div>
      </template>
    </div>
  </ScrollPanel>
</template>
