<script setup lang="ts">
import { reactive, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { NButton, NTabPane, NTabs } from 'naive-ui'
import { useVModel } from '@vueuse/core'
import { ProxyType } from '@/types/proxy'
import type { HysteriaProxy, XrayProxy } from '@/types/proxy'
import { createHysteriaProxy, createXrayProxy } from '@/apis/proxy'
import XrayForm from '@/views/proxy/form/XrayForm.vue'
import HysteriaForm from '@/views/proxy/form/HysteriaForm.vue'

const props = withDefaults(defineProps<Props>(), { showModal: false, currentTab: ProxyType.Hysteria })

const emits = defineEmits<Emits>()

const { t } = useI18n()

interface Props {
  showModal: boolean
  currentTab: ProxyType
}

interface Emits {
  (e: 'insertSubmit', tab: ProxyType): void
}

const showInsertModal = useVModel(props, 'showModal')

const activeTab = ref<ProxyType>(props.currentTab)

const defaultHysteriaForm: HysteriaProxy = {
  name: '',
  server: '',
  auth: '',
  bandwidth: {
    up: '10 mbps',
    down: '100 mbps',
  },
  tls: {
    sni: '',
    insecure: true,
  },
}

const hysteriaFormState = reactive<HysteriaProxy>({ ...defaultHysteriaForm })

const defaultXrayForm: XrayProxy = {
  id: 0,
  name: '',
  protocol: 'vmess',
  uuid: '',
  address: '',
  port: 443,
  streamSettings: {
    network: 'ws',
    security: 'none',
    tlsSettings: {
      serverName: '',
      allowInsecure: true,
    },
    wsSettings: {
      path: '',
      headers: {
        host: '',
      },
    },
    tcpSettings: {},
    http2Settings: {
      path: '',
      host: [''],
    },
    kcpSettings: {},
    grpcSettings: {},
  },
}

const xrayFormState = reactive<XrayProxy>({ ...defaultXrayForm })

async function onInsertSubmit() {
  if (activeTab.value === 'hysteria') {
    await createHysteriaProxy(hysteriaFormState)
    Object.assign(hysteriaFormState, defaultHysteriaForm)
  }
  else {
    await createXrayProxy(xrayFormState)
    Object.assign(xrayFormState, defaultXrayForm)
  }

  emits('insertSubmit', activeTab.value)
  showInsertModal.value = false
}

function onCancelInsert() {
  showInsertModal.value = false
}

watch(() => props.currentTab, (tab) => {
  activeTab.value = tab
})
</script>

<template>
  <n-modal
    v-model:show="showInsertModal"
    class="w-1/2 h-1/2"
    :mask-closable="false"
    transform-origin="center"
    preset="card"
    :title="t('proxy.addProxy.title')"
    size="huge"
    :bordered="false"
    :segmented="true"
  >
    <n-tabs
      v-model:value="activeTab"
      type="line"
      animated
    >
      <n-tab-pane
        name="hysteria"
        :tab="ProxyType.Hysteria"
      >
        <hysteria-form v-model:form="hysteriaFormState" />
      </n-tab-pane>
      <n-tab-pane
        name="xray"
        :tab="ProxyType.Xray"
      >
        <xray-form v-model:form="xrayFormState" />
      </n-tab-pane>
    </n-tabs>

    <template #footer>
      <div class="w-full flex flex-center gap-16">
        <n-button
          round
          @click="onCancelInsert"
        >
          {{ t('common.cancel') }}
        </n-button>
        <n-button
          round
          type="primary"
          @click="onInsertSubmit"
        >
          {{ t('common.add') }}
        </n-button>
      </div>
    </template>
  </n-modal>
</template>

<style>
.n-card-header {
  padding: 12px 24px !important;
}

.n-card__content {
  padding: 0 24px !important;
}

.n-card__footer {
  padding: 12px 24px !important;
}
</style>
