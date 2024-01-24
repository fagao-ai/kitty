<script setup lang="ts">
import { reactive, ref } from 'vue'
import { NButton, NForm, NFormItem, NInput, NTabPane, NTabs } from 'naive-ui'
import { useVModel } from '@vueuse/core'
import type { HysteriaProxy, XrayProxy } from '@/types/proxy'
import { invoke } from '@/utils/invoke'
import XrayView from '@/views/proxy/xray/XrayView.vue'

interface Props {
  showModal: boolean
}

const activeTab = ref('hysteria') // 跟踪当前活动的表单


const props = withDefaults(defineProps<Props>(), { showModal: false })

const emits = defineEmits<Emits>()

const hysteriaFormState = reactive<HysteriaProxy>({
  name: '名称',
  server: 'ip:port',
  auth: 'password',
  bandwidth: {
    up: '10 mbps',
    down: '100 mbps',
  },
  tls: {
    sni: 'bing.com',
    insecure: true,
  },
})

const xrayFormState = reactive<XrayProxy>({
  id: 0,
  name: '名称',
  protocol: 'VLESS',
  uuid: 'xxxx-xxxx-xxxx-xxxx',
  address: 'ip',
  port: 443,
  streamSettings: {
    network: '',
    security: 'tls',
    tlsSettings: {
      allowInsecure: true,
      serverName: 'bing.com',
    },
    // ws
    wsSettings: {
      path: '',
      headers: { Host: '' },
    },
    // tcp
    tcpSettings: {},
    // http2
    http2Settings: {
      host: [],
      path: '',
    },
    // grpc
    grpcSettings: {},
    // kcp
    kcpSettings: {},
  } as any,
  subscribeId: 0,
})

interface Emits {
  (e: 'insertSubmit'): void
}

const showInsertModal = useVModel(props, 'showModal')

async function onInsertSubmit() {
  if (activeTab.value == "hysteria") {
    await invoke('add_hy_item', { record: hysteriaFormState })
    console.log(JSON.stringify(hysteriaFormState));
  }
  else {
    // await invoke('add_hy_item', { record: hysteriaFormState })
    console.log(JSON.stringify(xrayFormState));
  }
  emits('insertSubmit')
  showInsertModal.value = false
}

function onCancelInsert() {
  showInsertModal.value = false
}
</script>

<template>
  <n-modal v-model:show="showInsertModal" class="w-1/2 h-1/2" :mask-closable="false" preset="card" title="添加代理"
    size="huge" :bordered="false" :segmented="true">
    <n-tabs type="line" animated v-model:value="activeTab">

      <n-tab-pane name="hysteria" tab="hysteria">
        <n-form :model="hysteriaFormState" size="medium" label-placement="left" label-width="auto">
          <n-form-item label="代理名称" path="name">
            <n-input v-model:value="hysteriaFormState.name" />
          </n-form-item>
          <n-form-item label="服务地址" path="server">
            <n-input v-model:value="hysteriaFormState.server" />
          </n-form-item>
          <n-form-item label="认证" path="auth">
            <n-input v-model:value="hysteriaFormState.auth" placeholder="认证密码" />
          </n-form-item>
          <n-form-item label="上行" path="bandwidth.up">
            <n-input v-model:value="hysteriaFormState.bandwidth.up" />
          </n-form-item>
          <n-form-item label="下行" path="bandwidth.down">
            <n-input v-model:value="hysteriaFormState.bandwidth.down" />
          </n-form-item>
          <n-form-item label="sni" path="tls.sni">
            <n-input v-model:value="hysteriaFormState.tls.sni" />
          </n-form-item>
          <n-form-item label="安全连接" path="tls.insecure">
            <n-switch v-model:value="hysteriaFormState.tls.insecure" />
          </n-form-item>
        </n-form>
      </n-tab-pane>
      <n-tab-pane name="Xray" tab="Xray">
        <xray-view v-model:form="xrayFormState" />
      </n-tab-pane>
    </n-tabs>

    <template #footer>
      <div class="w-full flex flex-center gap-16">
        <n-button round @click="onCancelInsert">
          取消
        </n-button>
        <n-button round type="primary" @click="onInsertSubmit">
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
