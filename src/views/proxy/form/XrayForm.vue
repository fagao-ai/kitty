<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { NButton, NForm, NFormItem, NInput, NInputNumber, NScrollbar } from 'naive-ui'
import { useVModel } from '@vueuse/core'
import type { XrayProxy } from '@/types/proxy'

const props = defineProps<Props>()

const { t } = useI18n()

interface Props {
  form: XrayProxy
}
const formState = useVModel(props, 'form')

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
  <n-scrollbar style="height: 100%;">
    <n-form
      :model="formState"
      size="medium"
      label-placement="left"
      label-width="auto"
    >
      <n-form-item
        :label="t('proxy.xray.proxyName')"
        path="name"
      >
        <n-input v-model:value="formState.name" />
      </n-form-item>
      <n-form-item
        :label="t('proxy.xray.protocol')"
        path="protocol"
      >
        <n-select
          v-model:value="formState.protocol"
          :options="[{ label: 'vless', value: 'vless' }, { label: 'vmess', value: 'vmess' }, { label: 'trojan', value: 'trojan' }]"
        />
      </n-form-item>
      <n-form-item
        label="uuid"
        path="uuid"
      >
        <n-input
          v-model:value="formState.uuid"
          placeholder="xxxx-xxxx-xxxx-xxxx"
        />
      </n-form-item>
      <n-form-item
        :label="t('proxy.xray.address')"
        path="address"
      >
        <n-input
          v-model:value="formState.address"
          placeholder="www.example.com"
        />
      </n-form-item>
      <n-form-item
        :label="t('proxy.xray.port')"
        path="port"
      >
        <n-input-number
          v-model:value="formState.port"
          type="text"
          :show-button="false"
          :max="65535"
          :min="1"
        />
      </n-form-item>
      <n-form-item :label="t('proxy.xray.network')">
        <n-select
          v-model:value="formState.streamSettings.network"
          :options="streamSettingOptions"
        />
      </n-form-item>
      <n-form-item
        :label="t('proxy.xray.streamSetting.security')"
        path="streamSetting.security"
      >
        <n-select
          v-model:value="formState.streamSettings.security"
          :options="[{ label: 'none', value: 'none' }, { label: 'tls', value: 'tls' }, { label: 'reality', value: 'reality' }]"
        />
      </n-form-item>
      <n-form-item
        :label="t('proxy.xray.streamSetting.tlsSettings.allowInsecure')"
        path="streamSetting.tlsSettings.allowInsecure"
      >
        <n-switch
          v-model:value="formState.streamSettings.tlsSettings!.allowInsecure"
          size="medium"
        />
      </n-form-item>
      <n-form-item
        :label="t('proxy.xray.streamSetting.tlsSettings.serverName')"
        path="streamSetting.tlsSettings.serverName"
      >
        <n-input
          v-model:value="formState.streamSettings.tlsSettings!.serverName"
          placeholder="www.example.com"
        />
      </n-form-item>
      <template v-if="formState.streamSettings.network === 'ws'">
        <n-form-item
          :label="t('proxy.streamSetting.wsSettings.path')"
          path="streamSetting.wsSettings.path"
        >
          <n-input v-model:value="formState.streamSettings.wsSettings.path" />
        </n-form-item>
        <n-form-item
          :label="t('proxy.streamSetting.wsSettings.host')"
          path="streamSetting.wsSettings.headers.host"
        >
          <n-input v-model:value="formState.streamSettings.wsSettings.headers.host" />
        </n-form-item>
      </template>

      <template v-if="formState.streamSettings.network === 'http2'">
        <n-form-item
          :label="t('proxy.streamSetting.http2Settings.path')"
          path="streamSetting.http2Settings.path"
        >
          <n-input v-model:value="formState.streamSettings.http2Settings.path" />
        </n-form-item>
        <n-form-item
          v-for="(item, index) in formState.streamSettings.http2Settings.host"
          :key="index"
          :label="`${t('proxy.xray.streamSetting.http2Settings.headers.host')}${index + 1}`"
          :path="`streamSetting.http2Settings.headers.host[${index}]`"
        >
          <n-input v-model:value="formState.streamSettings.http2Settings.host[index]" />
          <n-button
            class="pl-3"
            @click="handleAddHttp2Host()"
          >
            +
          </n-button>
          <n-button
            class="pl-3"
            @click="handleRemoveHttp2Host(index)"
          >
            -
          </n-button>
        </n-form-item>
      </template>
    </n-form>
  </n-scrollbar>
</template>
