<script setup lang="ts">
import PrimeInputText from 'primevue/inputtext'
import PrimeToggleSwitch from 'primevue/toggleswitch'
import { useI18n } from 'vue-i18n'
import { useVModel } from '@vueuse/core'
import type { HysteriaProxy } from '@/types/proxy'

const props = defineProps<Props>()
const emits = defineEmits<Emits>()

const { t } = useI18n()

interface Props {
  form: HysteriaProxy
}

interface Emits {
  (e: 'update:form', value: HysteriaProxy): void
}

const formState = useVModel(props, 'form', emits)
</script>

<template>
  <div class="grid gap-4">
    <label class="flex flex-col gap-2">
      <span class="text-sm font-medium">{{ t('proxy.hysteria.proxyName') }}</span>
      <prime-input-text v-model="formState.name" />
    </label>
    <label class="flex flex-col gap-2">
      <span class="text-sm font-medium">{{ t('proxy.hysteria.server') }}</span>
      <prime-input-text
        v-model="formState.server"
        placeholder="ip:port"
      />
    </label>
    <label class="flex flex-col gap-2">
      <span class="text-sm font-medium">{{ t('proxy.hysteria.auth') }}</span>
      <prime-input-text
        v-model="formState.auth"
        :placeholder="t('proxy.hysteria.authPlaceholder')"
      />
    </label>
    <label class="flex flex-col gap-2">
      <span class="text-sm font-medium">{{ t('proxy.hysteria.bandwidth.uplink') }}</span>
      <prime-input-text v-model="formState.bandwidth.up" />
    </label>
    <label class="flex flex-col gap-2">
      <span class="text-sm font-medium">{{ t('proxy.hysteria.bandwidth.downlink') }}</span>
      <prime-input-text v-model="formState.bandwidth.down" />
    </label>
    <label class="flex flex-col gap-2">
      <span class="text-sm font-medium">sni</span>
      <prime-input-text
        v-model="formState.tls.sni"
        placeholder="bing.com"
      />
    </label>
    <div class="flex items-center justify-between gap-4 rounded-xl border border-slate-200 px-4 py-3 dark:border-slate-700">
      <span class="text-sm font-medium">{{ t('proxy.hysteria.tls.insecure') }}</span>
      <prime-toggle-switch v-model="formState.tls.insecure" />
    </div>
  </div>
</template>
