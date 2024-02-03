<script setup lang="ts">
import { NForm, NFormItem, NInput } from 'naive-ui'
import { useVModel } from '@vueuse/core'
import type { HysteriaProxy } from '@/types/proxy'

interface Props {
  form: HysteriaProxy
}

const props = defineProps<Props>()

const hysteriaFormState = useVModel(props, 'form')
</script>

<template>
  <n-form
    :model="hysteriaFormState"
    size="medium"
    label-placement="left"
    label-width="auto"
  >
    <n-form-item
      label="代理名称"
      path="name"
    >
      <n-input v-model:value="hysteriaFormState.name" />
    </n-form-item>
    <n-form-item
      label="服务地址"
      path="server"
    >
      <n-input
        v-model:value="hysteriaFormState.server"
        placeholder="ip:port"
      />
    </n-form-item>
    <n-form-item
      label="认证"
      path="auth"
    >
      <n-input
        v-model:value="hysteriaFormState.auth"
        placeholder="认证密码"
      />
    </n-form-item>
    <n-form-item
      label="上行"
      path="bandwidth.up"
    >
      <n-input v-model:value="hysteriaFormState.bandwidth.up" />
    </n-form-item>
    <n-form-item
      label="下行"
      path="bandwidth.down"
    >
      <n-input v-model:value="hysteriaFormState.bandwidth.down" />
    </n-form-item>
    <n-form-item
      label="sni"
      path="tls.sni"
    >
      <n-input
        v-model:value="hysteriaFormState.tls.sni"
        placeholder="bing.com"
      />
    </n-form-item>
    <n-form-item
      label="安全连接"
      path="tls.insecure"
    >
      <n-switch v-model:value="hysteriaFormState.tls.insecure" />
    </n-form-item>
  </n-form>
</template>
