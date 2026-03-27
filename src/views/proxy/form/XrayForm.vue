<script setup lang="ts">
import PrimeButton from 'primevue/button'
import PrimeInputNumber from 'primevue/inputnumber'
import PrimeInputText from 'primevue/inputtext'
import PrimeSelect from 'primevue/select'
import PrimeToggleSwitch from 'primevue/toggleswitch'
import { useI18n } from 'vue-i18n'
import { useVModel } from '@vueuse/core'
import type { XrayProxy } from '@/types/proxy'

const props = defineProps<Props>()
const emits = defineEmits<Emits>()

const { t } = useI18n()

interface Props {
  form: XrayProxy
}

interface Emits {
  (e: 'update:form', value: XrayProxy): void
}

const formState = useVModel(props, 'form', emits)

const streamSettingOptions = [{ label: 'WebSocket', value: 'ws' }, { label: 'Tcp', value: 'tcp' }, { label: 'http2', value: 'http2' }, { label: 'grpc', value: 'grpc' }, { label: 'kcp', value: 'kcp' }]

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
  <div class="h-full overflow-y-auto pr-2">
    <div class="grid gap-4">
      <label class="flex flex-col gap-2">
        <span class="text-sm font-medium">{{ t('proxy.xray.proxyName') }}</span>
        <prime-input-text v-model="formState.name" />
      </label>
      <label class="flex flex-col gap-2">
        <span class="text-sm font-medium">{{ t('proxy.xray.protocol') }}</span>
        <prime-select
          v-model="formState.protocol"
          :options="[{ label: 'vless', value: 'vless' }, { label: 'vmess', value: 'vmess' }, { label: 'trojan', value: 'trojan' }]"
          option-label="label"
          option-value="value"
        />
      </label>
      <label class="flex flex-col gap-2">
        <span class="text-sm font-medium">uuid</span>
        <prime-input-text
          v-model="formState.uuid"
          placeholder="xxxx-xxxx-xxxx-xxxx"
        />
      </label>
      <label class="flex flex-col gap-2">
        <span class="text-sm font-medium">{{ t('proxy.xray.address') }}</span>
        <prime-input-text
          v-model="formState.address"
          placeholder="www.example.com"
        />
      </label>
      <label class="flex flex-col gap-2">
        <span class="text-sm font-medium">{{ t('proxy.xray.port') }}</span>
        <prime-input-number
          v-model="formState.port"
          :max="65535"
          :min="1"
          input-class="w-full"
        />
      </label>
      <label class="flex flex-col gap-2">
        <span class="text-sm font-medium">{{ t('proxy.xray.network') }}</span>
        <prime-select
          v-model="formState.streamSettings.network"
          :options="streamSettingOptions"
          option-label="label"
          option-value="value"
        />
      </label>
      <label class="flex flex-col gap-2">
        <span class="text-sm font-medium">{{ t('proxy.xray.streamSetting.security') }}</span>
        <prime-select
          v-model="formState.streamSettings.security"
          :options="[{ label: 'none', value: 'none' }, { label: 'tls', value: 'tls' }, { label: 'reality', value: 'reality' }]"
          option-label="label"
          option-value="value"
        />
      </label>
      <div class="flex items-center justify-between gap-4 rounded-xl border border-slate-200 px-4 py-3 dark:border-slate-700">
        <span class="text-sm font-medium">{{ t('proxy.xray.streamSetting.tlsSettings.allowInsecure') }}</span>
        <prime-toggle-switch v-model="formState.streamSettings.tlsSettings!.allowInsecure" />
      </div>
      <label class="flex flex-col gap-2">
        <span class="text-sm font-medium">{{ t('proxy.xray.streamSetting.tlsSettings.serverName') }}</span>
        <prime-input-text
          v-model="formState.streamSettings.tlsSettings!.serverName"
          placeholder="www.example.com"
        />
      </label>

      <template v-if="formState.streamSettings.network === 'ws'">
        <label class="flex flex-col gap-2">
          <span class="text-sm font-medium">{{ t('proxy.streamSetting.wsSettings.path') }}</span>
          <prime-input-text v-model="formState.streamSettings.wsSettings.path" />
        </label>
        <label class="flex flex-col gap-2">
          <span class="text-sm font-medium">{{ t('proxy.streamSetting.wsSettings.host') }}</span>
          <prime-input-text v-model="formState.streamSettings.wsSettings.headers.host" />
        </label>
      </template>

      <template v-if="formState.streamSettings.network === 'http2'">
        <label class="flex flex-col gap-2">
          <span class="text-sm font-medium">{{ t('proxy.streamSetting.http2Settings.path') }}</span>
          <prime-input-text v-model="formState.streamSettings.http2Settings.path" />
        </label>
        <div
          v-for="(item, index) in formState.streamSettings.http2Settings.host"
          :key="index"
          class="flex flex-col gap-2"
        >
          <span class="text-sm font-medium">{{ `${t('proxy.xray.streamSetting.http2Settings.headers.host')}${index + 1}` }}</span>
          <div class="flex gap-2">
            <prime-input-text v-model="formState.streamSettings.http2Settings.host[index]" class="flex-1" />
            <prime-button icon="pi pi-plus" severity="secondary" variant="outlined" @click="handleAddHttp2Host()" />
            <prime-button icon="pi pi-minus" severity="secondary" variant="outlined" @click="handleRemoveHttp2Host(index)" />
          </div>
        </div>
      </template>
    </div>
  </div>
</template>
