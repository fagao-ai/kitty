<script setup lang="ts">
import { NButton } from 'naive-ui'
import { reactive, ref, watch } from 'vue'
import { ProxyType } from '@/types/proxy'
import AddProxyModal from '@/views/proxy/AddProxyModal.vue'
import type { ProxyCard as Card, HysteriaProxy } from '@/types/proxy'
import { proxyStore } from '@/views/proxy/store'
import HysteriaProxyView from '@/views/proxy/HysteriaProxy.vue'
import XrayProxy from '@/views/proxy/XrayProxy.vue'
import { batchGetProxy } from '@/apis/proxy'

const showInsertModal = ref(false)

const formValue = reactive<HysteriaProxy>({
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

const hysterias = ref<Card[]>([])
const xrays = ref<Card[]>([])

async function initHysteria() {
  hysterias.value = await batchGetProxy()
}

async function initXray() {
  xrays.value = []
}

watch(proxyStore, () => {
  if (proxyStore.value.currentProxy === ProxyType.Hysteria) {
    initHysteria()
    return
  }
  initXray()
}, { immediate: true, deep: true })
</script>

<template>
  <div class="flex flex-col w-full h-full space-y-4">
    <div class="h-8 flex justify-between items-center">
      <div class="text-primay text-2xl font-extrabold">
        Proxies
      </div>
      <div>
        <n-button
          round
          @click="showInsertModal = true"
        >
          add
        </n-button>
      </div>
    </div>
    <div class="h-8 flex justify-center items-center">
      <n-radio-group
        v-model:value="proxyStore.currentProxy"
        name="proxyGroup"
        :on-update-value="() => {}"
      >
        <n-radio-button class="w-20" :value="ProxyType.Hysteria">
          {{ ProxyType.Hysteria }}
        </n-radio-button>
        <n-radio-button class="w-20" :value="ProxyType.Xray">
          {{ ProxyType.Xray }}
        </n-radio-button>
      </n-radio-group>
    </div>
    <div class="flex-1 w-full">
      <hysteria-proxy-view v-if="proxyStore.currentProxy === ProxyType.Hysteria" :data="hysterias" />
      <xray-proxy v-if="proxyStore.currentProxy === ProxyType.Xray" :data="xrays" />
    </div>
  </div>
  <add-proxy-modal
    v-model:showModal="showInsertModal"
    :form-data="formValue"
    @insert-submit="batchGetProxy"
  />
</template>

<style lang="scss" scoped>
:deep(.n-radio-button) {
  .n-radio__label {
    @apply flex items-center justify-center;
  }
}
</style>
