<script setup lang="ts">
import PrimeButton from 'primevue/button'
import PrimeDialog from 'primevue/dialog'
import PrimeInputText from 'primevue/inputtext'
import PrimeTab from 'primevue/tab'
import PrimeTabList from 'primevue/tablist'
import PrimeTabPanel from 'primevue/tabpanel'
import PrimeTabPanels from 'primevue/tabpanels'
import PrimeTabs from 'primevue/tabs'
import { useVModel } from '@vueuse/core'
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
  <prime-dialog
    v-model:visible="showImportModal"
    modal
    :style="{ width: 'min(56rem, 92vw)' }"
    header="导入代理"
  >
    <prime-tabs v-model:value="activeTab">
      <prime-tab-list>
        <prime-tab value="hysteria" :disabled="disabledTab === ProxyType.Hysteria">
          {{ ProxyType.Hysteria }}
        </prime-tab>
        <prime-tab value="xray" :disabled="disabledTab === ProxyType.Xray">
          {{ ProxyType.Xray }}
        </prime-tab>
      </prime-tab-list>
      <prime-tab-panels class="pt-4">
        <prime-tab-panel value="hysteria" />
        <prime-tab-panel value="xray">
          <label class="flex flex-col gap-2">
            <span class="text-sm font-medium">订阅地址</span>
            <prime-input-text
              v-model="importProxyFormState.url"
              placeholder="https://example.com"
            />
          </label>
        </prime-tab-panel>
      </prime-tab-panels>
    </prime-tabs>

    <template #footer>
      <div class="w-full flex flex-center gap-3">
        <prime-button label="取消" severity="secondary" variant="outlined" @click="handleCancelImport" />
        <prime-button label="导入" @click="handleImport" />
      </div>
    </template>
  </prime-dialog>
</template>
