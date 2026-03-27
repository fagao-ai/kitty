<script setup lang="ts">
import PrimeButton from 'primevue/button'
import PrimeDialog from 'primevue/dialog'
import PrimeTab from 'primevue/tab'
import PrimeTabList from 'primevue/tablist'
import PrimeTabPanel from 'primevue/tabpanel'
import PrimeTabPanels from 'primevue/tabpanels'
import PrimeTabs from 'primevue/tabs'
import { ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
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

function createDefaultHysteriaForm(): HysteriaProxy {
  return {
    id: 0,
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
}

function createDefaultXrayForm(): XrayProxy {
  return {
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
}

const hysteriaFormState = ref<HysteriaProxy>(createDefaultHysteriaForm())
const xrayFormState = ref<XrayProxy>(createDefaultXrayForm())

async function onInsertSubmit() {
  if (activeTab.value === 'hysteria') {
    await createHysteriaProxy(hysteriaFormState.value)
    hysteriaFormState.value = createDefaultHysteriaForm()
  }
  else {
    await createXrayProxy(xrayFormState.value)
    xrayFormState.value = createDefaultXrayForm()
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
  <prime-dialog
    v-model:visible="showInsertModal"
    modal
    :style="{ width: 'min(56rem, 92vw)' }"
    :header="t('proxy.addProxy.title')"
  >
    <prime-tabs v-model:value="activeTab">
      <prime-tab-list>
        <prime-tab value="hysteria">
          {{ ProxyType.Hysteria }}
        </prime-tab>
        <prime-tab value="xray">
          {{ ProxyType.Xray }}
        </prime-tab>
      </prime-tab-list>
      <prime-tab-panels class="pt-4">
        <prime-tab-panel value="hysteria">
          <hysteria-form v-model:form="hysteriaFormState" />
        </prime-tab-panel>
        <prime-tab-panel value="xray">
          <xray-form v-model:form="xrayFormState" />
        </prime-tab-panel>
      </prime-tab-panels>
    </prime-tabs>

    <template #footer>
      <div class="w-full flex flex-center gap-3">
        <prime-button :label="t('common.cancel')" severity="secondary" variant="outlined" @click="onCancelInsert" />
        <prime-button :label="t('common.add')" @click="onInsertSubmit" />
      </div>
    </template>
  </prime-dialog>
</template>
