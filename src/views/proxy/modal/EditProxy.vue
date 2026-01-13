<script setup lang="ts">
import { reactive, watch, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import Dialog from 'primevue/dialog'
import Button from 'primevue/button'
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

// 使用 ref 而不是 reactive 以避免 const 赋值问题
const formState = ref<HysteriaProxy | XrayProxy>({ ...props.form })

watch(() => props.form, (val) => {
  formState.value = { ...val }
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
  <Dialog
    v-model:visible="showEditModal"
    modal
    :header="t('proxy.editProxy')"
    :style="{ width: '50vw', height: '50vh' }"
    :closable="false"
  >
    <template v-if="proxyType === ProxyType.Hysteria">
      <hysteria-form v-model:form="(formState.value as HysteriaProxy)" />
    </template>
    <template v-if="proxyType === ProxyType.Xray && Object.keys(formState.value).length > 0">
      <xray-form v-model:form="(formState.value as XrayProxy)" />
    </template>

    <template #footer>
      <div class="w-full flex flex-center gap-8">
        <Button
          :label="t('common.cancel')"
          severity="secondary"
          @click="emits('onCancelEdit')"
        />
        <Button
          :label="t('common.update')"
          @click="handleUpdateProxy"
        />
      </div>
    </template>
  </Dialog>
</template>
