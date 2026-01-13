<script setup lang="ts">
import Dialog from 'primevue/dialog'
import { useVModel } from '@vueuse/core'
import InputText from 'primevue/inputtext'
import Tabs from 'primevue/tabs'
import TabList from 'primevue/tablist'
import Tab from 'primevue/tab'
import TabPanels from 'primevue/tabpanels'
import TabPanel from 'primevue/tabpanel'
import Button from 'primevue/button'
import { reactive, ref, watch } from 'vue'
import { createImportProxy } from '@/apis/proxy'
import type { ImportProxy } from '@/types/proxy'
import { ProxyType } from '@/types/proxy'

interface Props {
  showModal: boolean
  currentTab: ProxyType
  disabledTab?: ProxyType
}

interface Emits {
  (e: 'onImport', tab: ProxyType): void
}

const props = withDefaults(defineProps<Props>(), { showModal: false, currentTab: ProxyType.Xray })

const emits = defineEmits<Emits>()

const showImportModal = useVModel(props, 'showModal')

const activeTab = ref<ProxyType>(props.currentTab)

const defaultImportProxyForm: ImportProxy = {
  url: '',
}

const importProxyFormState = reactive<ImportProxy>({ ...defaultImportProxyForm })

async function handleImport() {
  if (activeTab.value === 'xray') {
    await createImportProxy(importProxyFormState)
    Object.assign(importProxyFormState, defaultImportProxyForm)
  }
  emits('onImport', activeTab.value)
  showImportModal.value = false
}

function handleCancelImport() {
  showImportModal.value = false
}

watch(() => props.currentTab, (tab) => {
  activeTab.value = tab
})
</script>

<template>
  <Dialog
    v-model:visible="showImportModal"
    modal
    header="导入代理"
    :style="{ width: '50vw', height: '50vh' }"
    :closable="false"
  >
    <Tabs v-model:value="activeTab">
      <TabList>
        <Tab value="hysteria" :disabled="disabledTab === ProxyType.Hysteria">
          {{ ProxyType.Hysteria }}
        </Tab>
        <Tab value="xray" :disabled="disabledTab === ProxyType.Xray">
          {{ ProxyType.Xray }}
        </Tab>
      </TabList>
      <TabPanels>
        <TabPanel value="hysteria">
          <!-- Hysteria 不支持导入 -->
        </TabPanel>
        <TabPanel value="xray">
          <div class="flex flex-col gap-2">
            <label class="font-semibold text-sm">订阅地址</label>
            <InputText
              v-model="importProxyFormState.url"
              placeholder="https://example.com"
            />
          </div>
        </TabPanel>
      </TabPanels>
    </Tabs>

    <template #footer>
      <div class="w-full flex flex-center gap-8">
        <Button
          label="取消"
          severity="secondary"
          @click="handleCancelImport"
        />
        <Button
          label="导入"
          @click="handleImport"
        />
      </div>
    </template>
  </Dialog>
</template>
