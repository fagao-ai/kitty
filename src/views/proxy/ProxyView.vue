<script setup lang="ts">
import { NButton, NIcon, useMessage } from 'naive-ui'
import { computed, onMounted, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { ProxyType } from '@/types/proxy'
import AddProxy from '@/views/proxy/modal/AddProxy.vue'
import type { ProxyCard as Card, HysteriaProxy, ProxyDelayInfo, XrayProxy } from '@/types/proxy'
import { proxyStore } from '@/views/proxy/store'
import ProxyCardList from '@/components/ProxyCardList.vue'
import {
  getActiveProxy,
  getAllHysterias,
  getAllXraies,
  getProxyByIdAndType,
  switchToProxy,
  xrayProxiedDelay,
} from '@/apis/proxy'
import ImportProxy from '@/views/proxy/modal/ImportProxy.vue'
import EditProxy from '@/views/proxy/modal/EditProxy.vue'
import HeaderBar from '@/components/HeaderBar.vue'
import { useSubscriptionAutoUpdate } from '@/tools/autoUpdateHook'

defineEmits<{
  toggleMobileMenu: []
}>()
const { t } = useI18n()
const message = useMessage()

const showInsertModal = ref(false)
const showImportModal = ref(false)

// Speed test state
const isTestingSpeed = ref(false)

// Switching proxy state
const switchingProxyId = ref<number | null>(null)

// Save original proxy data for speed test
const hysteriaProxiesData = ref<HysteriaProxy[]>([])
const xrayProxiesData = ref<XrayProxy[]>([])

// Unified card list (merged hysteria and xray)
const allCards = ref<Card[]>([])

// Computed: add active state
const cards = computed(() => {
  return allCards.value.map(card => ({
    ...card,
    isActive: card.id === proxyStore.value.activeProxyId
      && card.type === proxyStore.value.activeProxyType,
  }))
})

// Initialize all proxies (merge hysteria and xray)
async function initAllProxies() {
  const [hysteriaProxies, xrayProxies] = await Promise.all([
    getAllHysterias(),
    getAllXraies(),
  ])

  // Save original data for speed test
  hysteriaProxiesData.value = hysteriaProxies
  xrayProxiesData.value = xrayProxies

  const hysteriaCards: Card[] = hysteriaProxies.map(item => ({
    id: item.id!,
    type: ProxyType.Hysteria,
    name: item.name,
    tag: 'hysteria',
    delay: 200,
    protocol: 'TCP',
    source: 'manual',
  }))

  const xrayCards: Card[] = xrayProxies.map(item => ({
    id: item.id!,
    type: ProxyType.Xray,
    name: item.name,
    tag: item.protocol,
    delay: 0,
    protocol: item.streamSettings.network,
    source: item.subscribeId ? 'subscription' : 'manual',
  }))

  allCards.value = [...hysteriaCards, ...xrayCards]
}

// Get currently active proxy
async function fetchActiveProxy() {
  const activeProxy = await getActiveProxy()
  if (activeProxy) {
    proxyStore.value.activeProxyId = activeProxy.id
    proxyStore.value.activeProxyType = activeProxy.proxyType as ProxyType
  }
}

// Single click to switch proxy
async function handleCardClick(id: number, proxyType: ProxyType) {
  switchingProxyId.value = id
  try {
    await switchToProxy(id, proxyType)
    proxyStore.value.activeProxyId = id
    proxyStore.value.activeProxyType = proxyType
    message.success('Switched successfully')
  }
  catch (e: any) {
    // If switching failed, refresh the proxy list as it might be stale
    await initAllProxies()
    await fetchActiveProxy()
    message.error(`Switch failed: ${e?.message || 'Unknown error'}. Proxy list refreshed.`)
  }
  finally {
    switchingProxyId.value = null
  }
}

// Double click to edit
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

async function handleUpdatedProxy(_proxyType: ProxyType) {
  await initAllProxies()
  showEditModal.value = false
  message.success(t('common.updateSuccess'))
}

// Parse Hysteria server field (format "example.com:port")
function parseHysteriaServer(server: string): { address: string, port: number } {
  const parts = server.split(':')
  return {
    address: parts[0] || '',
    port: parts[1] ? Number.parseInt(parts[1], 10) : 443,
  }
}

// Batch test all proxies speed
async function testAllProxiesSpeed() {
  isTestingSpeed.value = true
  try {
    const delayInfos: ProxyDelayInfo[] = []

    for (const card of allCards.value) {
      if (card.type === ProxyType.Xray) {
        const xray = xrayProxiesData.value.find(p => p.id === card.id)
        if (xray) {
          delayInfos.push({
            id: card.id,
            address: xray.address,
            port: xray.port,
            proxy_type: 'Xray',
          })
        }
      }
      else {
        const hysteria = hysteriaProxiesData.value.find(p => p.id === card.id)
        if (hysteria) {
          const { address, port } = parseHysteriaServer(hysteria.server)
          delayInfos.push({
            id: card.id,
            address,
            port,
            proxy_type: 'Hysteria2',
          })
        }
      }
    }

    const delayResults = await xrayProxiedDelay(delayInfos)

    allCards.value = allCards.value.map((card) => {
      const delay = delayResults[card.id] ?? delayResults[String(card.id)] ?? 9999
      return { ...card, delay }
    })

    // Sort cards by delay in ascending order (smallest delay first)
    allCards.value.sort((a, b) => a.delay - b.delay)

    message.success(`测速完成，测试了 ${delayInfos.length} 个节点`)
  }
  catch (e: any) {
    message.error(`测速失败: ${e?.message || '未知错误'}`)
  }
  finally {
    isTestingSpeed.value = false
  }
}

// Subscription update logic
const { updateStatus } = useSubscriptionAutoUpdate()

watch(updateStatus, async (newStatus, oldStatus) => {
  if (oldStatus === void 0 && newStatus === 'running') {
    await initAllProxies()
  }
  else if (oldStatus === 'running' && newStatus === 'stop') {
    await initAllProxies()
  }
}, { immediate: true })

// Initialize on mount
onMounted(async () => {
  await initAllProxies()
  await fetchActiveProxy()
})
</script>

<template>
  <div class="flex flex-col w-full h-full gap-y-4">
    <header-bar @toggle-mobile-menu="$emit('toggleMobileMenu')">
      <template #mobile-menu-button>
        <n-icon size="24">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M3 12h18M3 6h18M3 18h18" stroke-linecap="round" stroke-linejoin="round" />
          </svg>
        </n-icon>
      </template>
      <template #title>
        {{ t('menubar.proxies') }}
      </template>
      <template #default>
        <n-button
          size="small"
          @click="showInsertModal = true"
        >
          {{ t('common.add') }}
        </n-button>
        <n-button
          size="small"
          @click="showImportModal = true"
        >
          {{ t('common.import') }}
        </n-button>
      </template>
      <template #mobile-actions>
        <n-button
          size="small"
          @click="showInsertModal = true"
        >
          <n-icon>
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M12 5v14M5 12h14" stroke-linecap="round" stroke-linejoin="round" />
            </svg>
          </n-icon>
        </n-button>
      </template>
    </header-bar>

    <!-- Removed protocol toggle radio buttons -->

    <div class="flex-1 w-full overflow-y-hidden">
      <proxy-card-list
        :data="cards"
        :switching-id="switchingProxyId"
        @dblclick="handleCardDblClick"
        @click="handleCardClick"
      />
    </div>

    <add-proxy
      v-model:show-modal="showInsertModal"
      :current-tab="ProxyType.Xray"
      @insert-submit="initAllProxies"
    />

    <import-proxy
      v-model:show-modal="showImportModal"
      :current-tab="ProxyType.Xray"
      :disabled-tab="ProxyType.Hysteria"
      @on-import="initAllProxies"
    />

    <edit-proxy
      v-model:show-modal="showEditModal"
      :proxy-type="editProxyType"
      :form="(editingProxy as HysteriaProxy | XrayProxy)"
      @on-cancel-edit="handleCancelEdit"
      @on-proxy-updated="handleUpdatedProxy"
    />

    <!-- Float button for speed test -->
    <n-float-button
      :right="24"
      :bottom="24"
      :top="undefined"
      :width="48"
      :height="48"
      class="z-50"
      @click="testAllProxiesSpeed"
    >
      <n-icon class="text-accent text-2xl">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          xmlns:xlink="http://www.w3.org/1999/xlink"
          viewBox="0 0 20 20"
        >
          <g fill="none">
            <path
              d="M6.19 2.77c.131-.456.548-.77 1.022-.77h5.25c.725 0 1.237.71 1.007 1.398l-.002.008L12.205 7h2.564c.947 0 1.407 1.144.767 1.811l-.004.004l-8.676 8.858c-.755.782-2.06.06-1.796-.996l1.17-4.679H4.963a1.062 1.062 0 0 1-1.022-1.354l2.25-7.873zM7.213 3a.062.062 0 0 0-.06.045l-2.25 7.874c-.01.04.02.08.06.08H6.87a.5.5 0 0 1 .485.62l-1.325 5.3a.086.086 0 0 0-.003.03c0 .004.002.008.003.011c.004.008.013.02.03.03c.018.01.034.01.042.01a.03.03 0 0 0 .01-.004a.087.087 0 0 0 .024-.018l.004-.004l8.675-8.856a.056.056 0 0 0 .017-.032a.084.084 0 0 0-.007-.044a.079.079 0 0 0-.025-.034c-.005-.004-.013-.008-.03-.008H11.5a.5.5 0 0 1-.472-.666l1.493-4.254a.062.062 0 0 0-.06-.08H7.212z"
              fill="currentColor"
            />
          </g>
        </svg>
      </n-icon>
    </n-float-button>
  </div>
</template>

<style lang="scss" scoped>
/* Removed radio button styles */
</style>
