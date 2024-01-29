<script setup lang="ts">
import { reactive, ref, watch } from 'vue'
import { NButton, NForm, NFormItem, NInput, NTabPane, NTabs } from 'naive-ui'
import { useVModel } from '@vueuse/core'
import { ProxyType } from '@/types/proxy'
import type { HysteriaProxy, XrayProxy } from '@/types/proxy'
import { createHysteriaProxy, createXrayProxy } from '@/apis/proxy'
import XrayView from '@/views/proxy/xray/XrayView.vue'

interface Props {
  showModal: boolean
  currentTab: ProxyType
}

interface Emits {
  (e: 'insertSubmit', tab: ProxyType): void
}

const props = withDefaults(defineProps<Props>(), { showModal: false, currentTab: ProxyType.Hysteria })

const emits = defineEmits<Emits>()

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
    title="添加代理"
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
        <n-form
          :model="hysteriaFormState"
          size="medium"
          label-placement="left"
          label-width="auto"
        >
          <n-form-item
            label="代理名称"
            path="name"
          >
            <n-input v-model:value="hysteriaFormState.name" />
          </n-form-item>
          <n-form-item
            label="服务地址"
            path="server"
          >
            <n-input
              v-model:value="hysteriaFormState.server"
              placeholder="ip:port"
            />
          </n-form-item>
          <n-form-item
            label="认证"
            path="auth"
          >
            <n-input
              v-model:value="hysteriaFormState.auth"
              placeholder="认证密码"
            />
          </n-form-item>
          <n-form-item
            label="上行"
            path="bandwidth.up"
          >
            <n-input v-model:value="hysteriaFormState.bandwidth.up" />
          </n-form-item>
          <n-form-item
            label="下行"
            path="bandwidth.down"
          >
            <n-input v-model:value="hysteriaFormState.bandwidth.down" />
          </n-form-item>
          <n-form-item
            label="sni"
            path="tls.sni"
          >
            <n-input
              v-model:value="hysteriaFormState.tls.sni"
              placeholder="bing.com"
            />
          </n-form-item>
          <n-form-item
            label="安全连接"
            path="tls.insecure"
          >
            <n-switch v-model:value="hysteriaFormState.tls.insecure" />
          </n-form-item>
        </n-form>
      </n-tab-pane>
      <n-tab-pane
        name="xray"
        :tab="ProxyType.Xray"
      >
        <xray-view v-model:form="xrayFormState" />
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
          添加
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
