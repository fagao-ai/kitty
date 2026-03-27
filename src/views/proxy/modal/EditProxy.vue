<script setup lang="ts">
import PrimeButton from 'primevue/button'
import PrimeDialog from 'primevue/dialog'
import { computed, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useVModel } from '@vueuse/core'
import type { HysteriaProxy, XrayProxy } from '@/types/proxy'
import { ProxyType } from '@/types/proxy'
import XrayForm from '@/views/proxy/form/XrayForm.vue'
import HysteriaForm from '@/views/proxy/form/HysteriaForm.vue'
import { updateHysteriaProxy, updateXrayProxy } from '@/apis/proxy'

const props = defineProps<Props>()

const emits = defineEmits<Emits>()

const { t } = useI18n()

interface Props {
  showModal: boolean
  proxyType: ProxyType
  form: HysteriaProxy | XrayProxy
}

interface Emits {
  (e: 'onCancelEdit'): void
  (e: 'onProxyUpdated', proxyType: ProxyType): void
}

const showEditModal = useVModel(props, 'showModal')

function cloneProxy<T extends HysteriaProxy | XrayProxy>(proxy: T): T {
  return structuredClone(proxy)
}

const formState = ref<HysteriaProxy | XrayProxy>(cloneProxy(props.form))

const hysteriaFormState = computed({
  get: () => formState.value as HysteriaProxy,
  set: (value) => { formState.value = value },
})

const xrayFormState = computed({
  get: () => formState.value as XrayProxy,
  set: (value) => { formState.value = value },
})

watch(() => props.form, (val) => {
  formState.value = cloneProxy(val)
})

async function handleUpdateProxy() {
  if (props.proxyType === ProxyType.Hysteria)
    await updateHysteriaProxy(formState.value as HysteriaProxy)
  else
    await updateXrayProxy(formState.value as XrayProxy)

  emits('onProxyUpdated', props.proxyType)
}
</script>

<template>
  <prime-dialog
    v-model:visible="showEditModal"
    modal
    :style="{ width: 'min(56rem, 92vw)' }"
    :header="t('proxy.editProxy')"
  >
    <template v-if="proxyType === ProxyType.Hysteria">
      <hysteria-form v-model:form="hysteriaFormState" />
    </template>
    <template v-if="proxyType === ProxyType.Xray && Object.keys(formState).length > 0">
      <xray-form v-model:form="xrayFormState" />
    </template>
    <template #footer>
      <div class="w-full flex flex-center gap-3">
        <prime-button :label="t('common.cancel')" severity="secondary" variant="outlined" @click="emits('onCancelEdit')" />
        <prime-button :label="t('common.update')" @click="handleUpdateProxy" />
      </div>
    </template>
  </prime-dialog>
</template>
