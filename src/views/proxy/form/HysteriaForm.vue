<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import InputText from 'primevue/inputtext'
import ToggleSwitch from 'primevue/toggleswitch'
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
  <div class="flex flex-col gap-4">
    <div class="flex flex-col gap-2">
      <label class="font-semibold text-sm">{{ t('proxy.hysteria.proxyName') }}</label>
      <InputText v-model="hysteriaFormState.name" />
    </div>

    <div class="flex flex-col gap-2">
      <label class="font-semibold text-sm">{{ t('proxy.hysteria.server') }}</label>
      <InputText
        v-model="hysteriaFormState.server"
        placeholder="ip:port"
      />
    </div>

    <div class="flex flex-col gap-2">
      <label class="font-semibold text-sm">{{ t('proxy.hysteria.auth') }}</label>
      <InputText
        v-model="hysteriaFormState.auth"
        :placeholder="t('proxy.hysteria.authPlaceholder')"
      />
    </div>

    <div class="flex flex-col gap-2">
      <label class="font-semibold text-sm">{{ t('proxy.hysteria.bandwidth.uplink') }}</label>
      <InputText v-model="hysteriaFormState.bandwidth.up" />
    </div>

    <div class="flex flex-col gap-2">
      <label class="font-semibold text-sm">{{ t('proxy.hysteria.bandwidth.downlink') }}</label>
      <InputText v-model="hysteriaFormState.bandwidth.down" />
    </div>

    <div class="flex flex-col gap-2">
      <label class="font-semibold text-sm">sni</label>
      <InputText
        v-model="hysteriaFormState.tls.sni"
        placeholder="bing.com"
      />
    </div>

    <div class="flex items-center gap-2">
      <ToggleSwitch v-model="hysteriaFormState.tls.insecure" />
      <label class="font-semibold text-sm cursor-pointer">{{ t('proxy.hysteria.tls.insecure') }}</label>
    </div>
  </div>
</template>
