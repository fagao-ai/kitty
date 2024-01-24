<script setup lang="ts">
import { NForm, NFormItem, NInput, NInputNumber, NScrollbar } from 'naive-ui'
import { useVModel } from '@vueuse/core'
import type { Http2ProtocolSetting, WebSocketProtocolSetting, Xray } from '@/models/xray'

type XrayForm = {
  [K in keyof Xray]: Xray[K];
}

interface Props {
  form: XrayForm
}

const props = defineProps<Props>()

const formState = useVModel(props, 'form')

const streamSettingOptions = [{ label: 'WebSocket', value: 'ws' }, { label: 'Tcp', value: 'tcp' }, { label: 'http2', value: 'http2' }, { label: 'grpc', value: 'grpc' }, { label: 'kcp', value: 'kcp' }]
</script>

<template>
  <n-scrollbar style="height: 100%;">
    <n-form :model="formState" size="medium" label-placement="left" label-width="auto">
      <n-form-item label="name" path="name">
        <n-input v-model:value="formState.name" />
      </n-form-item>
      <n-form-item label="protocol" path="protocol">
        <n-input v-model:value="formState.protocol" />
      </n-form-item>
      <n-form-item label="uuid" path="uuid">
        <n-input v-model:value="formState.uuid" />
      </n-form-item>
      <n-form-item label="address" path="address">
        <n-input v-model:value="formState.address" />
      </n-form-item>
      <n-form-item label="port" path="port">
        <n-input-number v-model:value="formState.port" type="text" :show-button="false" :max="65535" :min="1" />
      </n-form-item>
      <n-form-item label="network">
        <n-select v-model:value="formState.streamSettings.network" :options="streamSettingOptions" />
      </n-form-item>
      <n-form-item label="security" path="streamSetting.security">
        <n-select
          v-model:value="formState.streamSettings.security"
          :options="[{ label: 'none', value: 'none' }, { label: 'tls', value: 'tls' }, { label: 'reality', value: 'reality' }]"
        />
      </n-form-item>
      <n-form-item label="allow insecure" path="streamSetting.tlsSettings.allowInsecure">
        <n-switch v-model:value="formState.streamSettings.tlsSettings!.allowInsecure" size="medium" />
      </n-form-item>
      <n-form-item label="server name" path="streamSetting.tlsSettings.serverName">
        <n-input v-model:value="formState.streamSettings.tlsSettings!.serverName" />
      </n-form-item>
      <template v-if="formState.streamSettings.network === 'ws'">
        <n-form-item label="path" path="streamSetting.wsSettings.path">
          <n-input v-model:value="(formState.streamSettings.wsSettings as WebSocketProtocolSetting)!.path" />
        </n-form-item>
        <n-form-item label="host" path="streamSetting.wsSettings.headers.host">
          <n-input v-model:value="(formState.streamSettings.wsSettings as WebSocketProtocolSetting)!.headers.host" />
        </n-form-item>
      </template>

      <template v-if="formState.streamSettings.network === 'http2'">
        <n-form-item label="path" path="streamSetting.http2Settings.path">
          <n-input v-model:value="(formState.streamSettings.http2Settings as Http2ProtocolSetting)!.path" />
        </n-form-item>
        <n-form-item label="host" path="streamSetting.http2Settings.headers.host">
          <n-input v-model:value="(formState.streamSettings.http2Settings as Http2ProtocolSetting)!.host" />
        </n-form-item>
      </template>
    </n-form>
  </n-scrollbar>
</template>
