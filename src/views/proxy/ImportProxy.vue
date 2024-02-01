<script setup lang="ts">
import { useVModel } from '@vueuse/core'
import { NButton, NForm, NFormItem, NInput, NTabPane, NTabs } from 'naive-ui'
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
  (e: 'insertSubmit', tab: ProxyType): void
}

const props = withDefaults(defineProps<Props>(), { showModal: false, currentTab: ProxyType.Xray })

const emits = defineEmits<Emits>()

const showImportModal = useVModel(props, 'showModal')

const activeTab = ref<ProxyType>(props.currentTab)

const defaultImportProxyForm: ImportProxy = {
  url: '',
}

const importProxyFormState = reactive<ImportProxy>({ ...defaultImportProxyForm })

async function onInsertSubmit() {
  if (activeTab.value === 'xray') {
    await createImportProxy(importProxyFormState)
    Object.assign(importProxyFormState, defaultImportProxyForm)
  }
  emits('insertSubmit', activeTab.value)
  showImportModal.value = false
}

function onCancelInsert() {
  showImportModal.value = false
}

watch(() => props.currentTab, (tab) => {
  activeTab.value = tab
})
</script>

<template>
  <n-modal
    v-model:show="showImportModal"
    class="w-1/2 h-1/2"
    :mask-closable="false"
    transform-origin="center"
    preset="card"
    title="导入代理"
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
        :disabled="disabledTab === ProxyType.Hysteria"
      />
      <n-tab-pane
        name="xray"
        :tab="ProxyType.Xray"
        :disabled="disabledTab === ProxyType.Xray"
      >
        <n-form
          :model="importProxyFormState"
          size="medium"
          label-placement="left"
          label-width="auto"
        >
          <n-form-item
            label="订阅地址"
            path="url"
          >
            <n-input
              v-model:value="importProxyFormState.url"
              placeholder="https://example.com"
            />
          </n-form-item>
        </n-form>
      </n-tab-pane>
    </n-tabs>

    <template #footer>
      <div class="w-full flex flex-center gap-16">
        <n-button
          round
          @click="onCancelInsert"
        >
          取消
        </n-button>
        <n-button
          round
          type="primary"
          @click="onInsertSubmit"
        >
          导入
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
