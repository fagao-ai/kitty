<script setup lang="ts">
import { NButton } from 'naive-ui'
import { computed, ref, watch } from 'vue'
import { ProxyType } from '@/types/proxy'
import AddProxyModal from '@/views/proxy/AddProxyModal.vue'
import type { ProxyCard as Card } from '@/types/proxy'
import { proxyStore } from '@/views/proxy/store'
import ProxyCardList from '@/components/ProxyCardList.vue'
import { getAllHysterias, getAllXraies } from '@/apis/proxy'
import ImportProxy from '@/views/proxy/ImportProxy.vue'

const showInsertModal = ref(false)
const showImportModal = ref(false)

const hysteriaCards = ref<Card[]>([])
const xrayCards = ref<Card[]>([])
const cards = computed(() => {
  return proxyStore.value.currentProxy === ProxyType.Hysteria
    ? hysteriaCards.value
    : xrayCards.value
})

async function initHysteria() {
  const hysteriaProxies = await getAllHysterias()
  hysteriaCards.value = hysteriaProxies.map((item) => {
    return {
      id: item.id!,
      type: ProxyType.Hysteria,
      name: item.name,
      tag: 'hysteria',
      delay: 200, // TODO
      protocol: 'TCP',
    }
  })
}

async function initXray() {
  const xraies = await getAllXraies()
  xrayCards.value = xraies.map((item) => {
    return {
      id: item.id!,
      type: ProxyType.Xray,
      name: item.name,
      tag: item.protocol,
      delay: 200, // TODO
      protocol: item.streamSettings.network,
    }
  })
}

function handleGetAllProxyByType(proxyType: ProxyType) {
  if (proxyType === ProxyType.Hysteria) {
    initHysteria()
    return
  }
  initXray()
}

function handleCardDblClick(id: number, proxyType: ProxyType) {
  // eslint-disable-next-line no-console
  console.log(`id is ${id}, proxyType is ${proxyType}`)
}

watch(proxyStore, () => {
  handleGetAllProxyByType(proxyStore.value.currentProxy)
}, { immediate: true, deep: true })
</script>

<template>
  <div class="flex flex-col w-full h-full space-y-4">
    <div class="h-8 flex justify-between items-center">
      <div class="text-primay text-2xl font-extrabold">
        Proxies
      </div>
      <div class="flex space-x-3">
        <n-button
          round
          @click="showInsertModal = true"
        >
          add
        </n-button>
        <n-button
          round
          @click="showImportModal = true"
        >
          import
        </n-button>
      </div>
    </div>
    <div class="h-8 flex justify-center items-center">
      <n-radio-group
        v-model:value="proxyStore.currentProxy"
        name="proxyGroup"
        :on-update-value="() => { }"
      >
        <n-radio-button
          class="w-20"
          :value="ProxyType.Hysteria"
        >
          {{ ProxyType.Hysteria }}
        </n-radio-button>
        <n-radio-button
          class="w-20"
          :value="ProxyType.Xray"
        >
          {{ ProxyType.Xray }}
        </n-radio-button>
      </n-radio-group>
    </div>
    <div class="flex-1 w-full">
      <proxy-card-list
        :data="cards"
        @dblclick="handleCardDblClick"
      />
    </div>
  </div>
  <add-proxy-modal
    v-model:showModal="showInsertModal"
    :current-tab="proxyStore.currentProxy"
    @insert-submit="handleGetAllProxyByType"
  />

  <import-proxy
    v-model:showModal="showImportModal"
    :current-tab="ProxyType.Xray"
    :disabled-tab="ProxyType.Hysteria"
    @insert-submit="handleGetAllProxyByType"
  />
</template>

<style lang="scss" scoped>
:deep(.n-radio-button) {
  .n-radio__label {
    @apply flex items-center justify-center;
  }
}
</style>
