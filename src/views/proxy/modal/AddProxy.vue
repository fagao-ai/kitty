<script setup lang="ts">
import { ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import Dialog from 'primevue/dialog'
import Tabs from 'primevue/tabs'
import TabList from 'primevue/tablist'
import Tab from 'primevue/tab'
import TabPanels from 'primevue/tabpanels'
import TabPanel from 'primevue/tabpanel'
import ScrollPanel from 'primevue/scrollpanel'
import Button from 'primevue/button'
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

const hysteriaFormState = ref<HysteriaProxy>({ ...defaultHysteriaForm })

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

const xrayFormState = ref<XrayProxy>({ ...defaultXrayForm })

async function onInsertSubmit() {
  if (activeTab.value === 'hysteria') {
    await createHysteriaProxy(hysteriaFormState.value)
    hysteriaFormState.value = { ...defaultHysteriaForm }
  }
  else {
    await createXrayProxy(xrayFormState.value)
    xrayFormState.value = { ...defaultXrayForm }
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
  <Dialog
    v-model:visible="showInsertModal"
    modal
    :header="t('proxy.addProxy.title')"
    :style="{ width: '50vw', height: '50vh' }"
    :content-style="{ overflow: 'hidden' }"
    :closable="false"
  >
    <Tabs v-model:value="activeTab">
      <TabList>
        <Tab value="hysteria">
          {{ ProxyType.Hysteria }}
        </Tab>
        <Tab value="xray">
          {{ ProxyType.Xray }}
        </Tab>
      </TabList>
      <TabPanels>
        <TabPanel value="hysteria">
          <ScrollPanel style="max-height: 40vh">
            <hysteria-form v-model:form="hysteriaFormState" />
          </ScrollPanel>
        </TabPanel>
        <TabPanel value="xray">
          <ScrollPanel style="max-height: 40vh">
            <xray-form v-model:form="xrayFormState" />
          </ScrollPanel>
        </TabPanel>
      </TabPanels>
    </Tabs>

    <template #footer>
      <div class="w-full flex flex-center gap-8">
        <Button
          :label="t('common.cancel')"
          severity="secondary"
          @click="onCancelInsert"
        />
        <Button
          :label="t('common.add')"
          @click="onInsertSubmit"
        />
      </div>
    </template>
  </Dialog>
</template>
