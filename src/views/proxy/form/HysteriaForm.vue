<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { NForm, NFormItem, NInput } from 'naive-ui'
import { useVModel } from '@vueuse/core'
import type { HysteriaProxy } from '@/types/proxy'

const props = defineProps<Props>()

const { t } = useI18n()

interface Props {
  form: HysteriaProxy
}

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
      :label="t('proxy.hysteria.proxyName')"
      path="name"
    >
      <n-input v-model:value="hysteriaFormState.name" />
    </n-form-item>
    <n-form-item
      :label="t('proxy.hysteria.server')"
      path="server"
    >
      <n-input
        v-model:value="hysteriaFormState.server"
        placeholder="ip:port"
      />
    </n-form-item>
    <n-form-item
      :label="t('proxy.hysteria.auth')"
      path="auth"
    >
      <n-input
        v-model:value="hysteriaFormState.auth"
        :placeholder="t('proxy.hysteria.authPlaceholder')"
      />
    </n-form-item>
    <n-form-item
      :label="t('proxy.hysteria.bandwidth.uplink')"
      path="bandwidth.up"
    >
      <n-input v-model:value="hysteriaFormState.bandwidth.up" />
    </n-form-item>
    <n-form-item
      :label="t('proxy.hysteria.bandwidth.downlink')"
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
      :label="t('proxy.hysteria.tls.insecure')"
      path="tls.insecure"
    >
      <n-switch v-model:value="hysteriaFormState.tls.insecure" />
    </n-form-item>
  </n-form>
</template>
