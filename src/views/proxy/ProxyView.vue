<script setup lang="ts">
import { NButton, NIcon, useMessage } from 'naive-ui'
import { computed, h, onMounted, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import type { Component, VNode } from 'vue'
import {
  LogoDocker,
  LogoGithub,
  LogoGoogle,
  LogoTwitch,
  LogoTwitter,
  LogoYoutube,
} from '@vicons/ionicons5'
import { ProxyType } from '@/types/proxy'
import AddProxy from '@/views/proxy/modal/AddProxy.vue'
import type { ProxyCard as Card, HysteriaProxy, XrayProxy } from '@/types/proxy'
import { proxyStore } from '@/views/proxy/store'
import ProxyCardList from '@/components/ProxyCardList.vue'
import {
  getAllHysterias,
  getAllXraies,
  getProxyByIdAndType,
  getActiveProxy,
  switchToProxy,
  currentProxyDelay,
} from '@/apis/proxy'
import ImportProxy from '@/views/proxy/modal/ImportProxy.vue'
import EditProxy from '@/views/proxy/modal/EditProxy.vue'
import HeaderBar from '@/components/HeaderBar.vue'
import { getProtocolShortName } from '@/utils/proxy'
import { useSubscriptionAutoUpdate } from '@/tools/autoUpdateHook'
import { settingStore } from '@/views/setting/store'

const { t } = useI18n()
const message = useMessage()

const showInsertModal = ref(false)
const showImportModal = ref(false)

// Unified card list (merged hysteria and xray)
const allCards = ref<Card[]>([])

// Computed: add protocol short name and active state
const cards = computed(() => {
  return allCards.value.map(card => ({
    ...card,
    protocolShortName: getProtocolShortName(card.tag, card.type),
    isActive: card.id === proxyStore.value.activeProxyId &&
              card.type === proxyStore.value.activeProxyType,
  }))
})

// Initialize all proxies (merge hysteria and xray)
async function initAllProxies() {
  const [hysteriaProxies, xrayProxies] = await Promise.all([
    getAllHysterias(),
    getAllXraies(),
  ])

  const hysteriaCards: Card[] = hysteriaProxies.map(item => ({
    id: item.id!,
    type: ProxyType.Hysteria,
    name: item.name,
    tag: 'hysteria',
    delay: 200,
    protocol: 'TCP',
  }))

  const xrayCards: Card[] = xrayProxies.map(item => ({
    id: item.id!,
    type: ProxyType.Xray,
    name: item.name,
    tag: item.protocol,
    delay: 0,
    protocol: item.streamSettings.network,
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
  try {
    await switchToProxy(id, proxyType)
    proxyStore.value.activeProxyId = id
    proxyStore.value.activeProxyType = proxyType
    message.success('Switched successfully')
  } catch (e: any) {
    // If switching failed, refresh the proxy list as it might be stale
    await initAllProxies()
    await fetchActiveProxy()
    message.error(`Switch failed: ${e?.message || 'Unknown error'}. Proxy list refreshed.`)
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

async function handleUpdatedProxy(proxyType: ProxyType) {
  await initAllProxies()
  showEditModal.value = false
  message.success(t('common.updateSuccess'))
}

// Speed test
function renderIcon(icon: Component) {
  return () => {
    return h(NIcon, null, {
      default: () => h(icon),
    })
  }
}
const speeds = ref<{ label: string, key: string, url: string, icon: () => VNode, delay?: number }[]>([
  {
    label: 'Google',
    key: 'Google',
    url: 'https://www.google.com',
    icon: renderIcon(LogoGoogle),
  },
  {
    label: 'Github',
    key: 'Github',
    url: 'https://www.github.com',
    icon: renderIcon(LogoGithub),
  },
  {
    label: 'Docker',
    key: 'Docker',
    url: 'https://registry-1.docker.io/v2/',
    icon: renderIcon(LogoDocker),
  },
  {
    label: 'Youtube',
    key: 'Youtube',
    url: 'https://www.youtube.com',
    icon: renderIcon(LogoYoutube),
  },
  {
    label: 'X',
    key: 'X',
    url: 'https://www.x.com',
    icon: renderIcon(LogoTwitter),
  },
  {
    label: 'Twitch',
    key: 'Twitch',
    url: 'https://www.twitch.tv',
    icon: renderIcon(LogoTwitch),
  },
])

async function onShowSpeed() {
  const proxyUrl = `http://127.0.0.1:${settingStore.value.port}`
  speeds.value.forEach((item) => {
    currentProxyDelay(proxyUrl, item.url).then((delay) => {
      item.delay = delay
      item.label = `${delay}ms`
    })
  })
}

// Subscription update logic
const { updateStatus } = useSubscriptionAutoUpdate()

watch(updateStatus, async (newStatus, oldStatus) => {
  if (oldStatus === void 0 && newStatus === 'running') {
    await initAllProxies()
  } else if (oldStatus === 'running' && newStatus === 'stop') {
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

    <!-- Removed protocol toggle radio buttons -->

    <div class="flex-1 w-full overflow-y-hidden">
      <proxy-card-list
        :data="cards"
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

    <!-- Removed v-if condition from float button since we no longer need to distinguish protocols -->
    <n-float-button
      :right="20"
      :top="70"
      :width="40"
      :height="40"
    >
      <n-dropdown
        trigger="hover"
        :options="speeds"
        @show="onShowSpeed"
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
              />
            </g>
          </svg>
        </n-icon>
      </n-dropdown>
    </n-float-button>
  </div>
</template>

<style lang="scss" scoped>
/* Removed radio button styles */
</style>
