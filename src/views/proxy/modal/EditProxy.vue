<script setup lang="ts">
import { reactive, watch } from 'vue'
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

const formState = reactive({ ...props.form })

watch(() => props.form, (val) => {
  Object.assign(formState, val)
})

async function handleUpdateProxy() {
  if (props.proxyType === ProxyType.Hysteria)
    await updateHysteriaProxy(formState as HysteriaProxy)
  else
    await updateXrayProxy(formState as XrayProxy)

  emits('onProxyUpdated', props.proxyType)
}
</script>

<template>
  <n-modal
    v-model:show="showEditModal"
    class="w-full h-full sm:w-[90%] sm:h-auto md:w-3/4 lg:w-1/2"
    :mask-closable="false"
    transform-origin="center"
    preset="card"
    :title="t('proxy.editProxy')"
    size="huge"
    :bordered="false"
    :segmented="true"
  >
    <template v-if="proxyType === ProxyType.Hysteria">
      <hysteria-form v-model:form="(formState as HysteriaProxy)" />
    </template>
    <template v-if="proxyType === ProxyType.Xray && Object.keys(formState).length > 0">
      <xray-form v-model:form="(formState as XrayProxy)" />
    </template>
    <template #footer>
      <div class="w-full flex flex-center gap-3">
        <n-button
          @click="emits('onCancelEdit')"
        >
          {{ t('common.cancel') }}
        </n-button>
        <n-button
          type="primary"
          @click="handleUpdateProxy"
        >
          {{ t('common.update') }}
        </n-button>
      </div>
    </template>
  </n-modal>
</template>

<style>
.n-modal {
  border-radius: 12px;
}

.n-card-header {
  padding: 20px 24px !important;
  border-bottom: 1px solid var(--n-border-color);
}

.n-card__content {
  padding: 24px !important;
}

.n-card__footer {
  padding: 16px 24px !important;
  border-top: 1px solid var(--n-border-color);
}
</style>
