<script setup lang="ts">
import { NButton, useMessage } from 'naive-ui'
import { computed, onUnmounted, ref, watch, unref } from 'vue'
import { useI18n } from 'vue-i18n'
import { ProxyType } from '@/types/proxy'
import AddProxy from '@/views/proxy/modal/AddProxy.vue'
import type { ProxyCard as Card, HysteriaProxy, XrayProxy } from '@/types/proxy'
import { proxyStore } from '@/views/proxy/store'
import ProxyCardList from '@/components/ProxyCardList.vue'
import { getAllHysterias, getAllXraies, getProxyByIdAndType, xrayProxiedDelay } from '@/apis/proxy'
import ImportProxy from '@/views/proxy/modal/ImportProxy.vue'
import EditProxy from '@/views/proxy/modal/EditProxy.vue'
import HeaderBar from '@/components/HeaderBar.vue'
import { useSubscriptionAutoUpdate } from '@/tools/autoUpdateHook'

const { t } = useI18n()
const message = useMessage()

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
  const delay_map = await xrayProxiedDelay(xraies.map((item) => {
    return { id: item.id, address: item.address, port: item.port }
  }))
  xrayCards.value = xraies.map((item) => {
    return {
      id: item.id!,
      type: ProxyType.Xray,
      name: item.name,
      tag: item.protocol,
      delay: delay_map[item.id!] ?? 9999,
      protocol: item.streamSettings.network,
    }
  }).sort((a, b) => a.delay - b.delay)
}

async function handleProxiesDelay() {
  initXray()
}

function handleGetAllProxyByType(proxyType: ProxyType) {
  if (proxyType === ProxyType.Hysteria) {
    initHysteria()
    return
  }
  initXray()
}

const unwatchProxyStore = watch(proxyStore, () => {
  handleGetAllProxyByType(proxyStore.value.currentProxy)
}, { immediate: false, deep: true })

// edit proxy
const showEditModal = ref(false)
const editingProxy = ref<Partial<HysteriaProxy | XrayProxy>>({})
const editProxyType = ref<ProxyType>(ProxyType.Hysteria)

async function handleCardDblClick(id: number, proxyType: ProxyType) {
  const res = await getProxyByIdAndType(id, proxyType)
  if (!res) {
    message.error('Invalid proxy, please check')
    return
  }
  editingProxy.value = res
  editProxyType.value = proxyType
  showEditModal.value = true
}

function handleCancelEdit() {
  showEditModal.value = false
  editingProxy.value = {}
}

const { updateStatus } = useSubscriptionAutoUpdate()

const unwatchUpdateStatus = watch(updateStatus, async (newStatus, oldStatus) => {
  if (oldStatus === void 0 && newStatus === 'running') {
    handleGetAllProxyByType(ProxyType.Xray)
  }
  else if (oldStatus === 'running' && newStatus === 'stop') {
    handleGetAllProxyByType(ProxyType.Xray)
  }
}, { immediate: true })

onUnmounted(() => {
  unwatchProxyStore()
  unwatchUpdateStatus()
})

async function handleUpdatedProxy(proxyType: ProxyType) {
  handleGetAllProxyByType(proxyType)
  showEditModal.value = false
  message.success(t('common.updateSuccess'))
}
</script>

<template>
  <div class="flex flex-col w-full h-full gap-y-4">
    <header-bar>
      <template #title>
        {{ t('menubar.proxies') }}
      </template>
      <template #default>
        <n-button
          round
          size="small"
          @click="showInsertModal = true"
        >
          {{ t('common.add') }}
        </n-button>
        <n-button
          round
          size="small"
          @click="showImportModal = true"
        >
          {{ t('common.import') }}
        </n-button>
      </template>
    </header-bar>
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
    <div class="flex-1 w-full overflow-y-hidden">
      <proxy-card-list
        :data="cards"
        @dblclick="handleCardDblClick"
      />
    </div>
    <add-proxy
      v-model:showModal="showInsertModal"
      :current-tab="proxyStore.currentProxy"
      @insert-submit="handleGetAllProxyByType"
    />

    <import-proxy
      v-model:showModal="showImportModal"
      :current-tab="ProxyType.Xray"
      :disabled-tab="ProxyType.Hysteria"
      @on-import="handleGetAllProxyByType"
    />
    <edit-proxy
      v-model:show-modal="showEditModal"
      :proxy-type="editProxyType"
      :form="(editingProxy as HysteriaProxy | XrayProxy)"
      @on-cancel-edit="handleCancelEdit"
      @on-proxy-updated="handleUpdatedProxy"
    />
    <n-float-button
      v-if="proxyStore.currentProxy === ProxyType.Xray"
      :right="20"
      :top="70"
      :width="40"
      :height="40"
      @click="handleProxiesDelay"
    >
      <n-icon class="text-[#63E2B7] text-2xl">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          xmlns:xlink="http://www.w3.org/1999/xlink"
          viewBox="0 0 20 20"
        >
          <g fill="none">
            <path
              d="M6.19 2.77c.131-.456.548-.77 1.022-.77h5.25c.725 0 1.237.71 1.007 1.398l-.002.008L12.205 7h2.564c.947 0 1.407 1.144.767 1.811l-.004.004l-8.676 8.858c-.755.782-2.06.06-1.796-.996l1.17-4.679H4.963a1.062 1.062 0 0 1-1.022-1.354l2.25-7.873zM7.213 3a.062.062 0 0 0-.06.045l-2.25 7.874c-.01.04.02.08.06.08H6.87a.5.5 0 0 1 .485.62l-1.325 5.3a.086.086 0 0 0-.003.03c0 .004.002.008.003.011c.004.008.013.02.03.03c.018.01.034.01.042.01a.03.03 0 0 0 .01-.004a.087.087 0 0 0 .024-.018l.004-.004l8.675-8.856a.056.056 0 0 0 .017-.032a.084.084 0 0 0-.007-.044a.079.079 0 0 0-.025-.034c-.005-.004-.013-.008-.03-.008H11.5a.5.5 0 0 1-.472-.666l1.493-4.254a.062.062 0 0 0-.06-.08H7.212z"
              fill="currentColor"
            ></path>
          </g>
        </svg>
      </n-icon>
    </n-float-button>
  </div>
</template>

<style lang="scss" scoped>
:deep(.n-radio-button) {
  --n-button-border-radius: 12px;

  .n-radio__label {
    @apply flex items-center justify-center;
  }
}
</style>
