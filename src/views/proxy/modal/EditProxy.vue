<script setup lang="ts">
import { reactive, watch } from 'vue'
import { useVModel } from '@vueuse/core'
import type { HysteriaProxy, XrayProxy } from '@/types/proxy'
import { ProxyType } from '@/types/proxy'
import XrayForm from '@/views/proxy/form/XrayForm.vue'
import HysteriaForm from '@/views/proxy/form/HysteriaForm.vue'


interface Props {
  showModal: boolean
  proxyType: ProxyType
  form: HysteriaProxy | XrayProxy
}

const props = defineProps<Props>()

const showEditModal = useVModel(props, 'showModal')

const formState = reactive({ ...props.form })

watch(() => props.form, (val) => {
  Object.assign(formState, val)
})
</script>

<template>
  <n-modal
    v-model:show="showEditModal"
    class="w-1/2 h-1/2"
    :mask-closable="false"
    transform-origin="center"
    preset="card"
    title="编辑代理"
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
  </n-modal>
</template>
