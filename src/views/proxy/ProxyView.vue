<script setup lang="ts">
import { computed, onUnmounted, ref, ref as vueRef, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import Button from 'primevue/button'
import SelectButton from 'primevue/selectbutton'
import Menu from 'primevue/menu'
import { useToast } from 'primevue/usetoast'

import { ProxyType } from '@/types/proxy'
import AddProxy from '@/views/proxy/modal/AddProxy.vue'
import type { ProxyCard as Card, HysteriaProxy, XrayProxy } from '@/types/proxy'
import { proxyStore } from '@/views/proxy/store'
import ProxyCardList from '@/components/ProxyCardList.vue'
import { currentProxyDelay, getAllHysterias, getAllXraies, getProxyByIdAndType, setProxy, xrayProxiedDelay } from '@/apis/proxy'
import ImportProxy from '@/views/proxy/modal/ImportProxy.vue'
import EditProxy from '@/views/proxy/modal/EditProxy.vue'
import HeaderBar from '@/components/HeaderBar.vue'
import { useSubscriptionAutoUpdate } from '@/tools/autoUpdateHook'
import { settingStore } from '@/views/setting/store'

const { t } = useI18n()
const toast = useToast()

const proxyTypeOptions = [
  { label: ProxyType.Hysteria, value: ProxyType.Hysteria },
  { label: ProxyType.Xray, value: ProxyType.Xray },
]

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

  if (settingStore.value.sysproxyFlag && xraies.length > 0) {
    await setProxy(false)
    await setProxy(true, xrayCards.value[0].id)
  }
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
    toast.add({ severity: 'error', summary: 'Error', detail: 'Invalid proxy, please check', life: 3000 })
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
  if (oldStatus === void 0 && newStatus === 'running')
    handleGetAllProxyByType(ProxyType.Xray)

  else if (oldStatus === 'running' && newStatus === 'stop')
    handleGetAllProxyByType(ProxyType.Xray)
}, { immediate: true })

onUnmounted(() => {
  unwatchProxyStore()
  unwatchUpdateStatus()
})

async function handleUpdatedProxy(proxyType: ProxyType) {
  handleGetAllProxyByType(proxyType)
  showEditModal.value = false
  toast.add({ severity: 'success', summary: 'Success', detail: t('common.updateSuccess'), life: 3000 })
}

interface SpeedItem {
  label: string
  key: string
  url: string
  delay?: number
  command?: () => void
}

const speeds = ref<SpeedItem[]>([
  {
    label: 'Google',
    key: 'Google',
    url: 'https://www.google.com',
  },
  {
    label: 'Github',
    key: 'Github',
    url: 'https://www.github.com',
  },
  {
    label: 'Docker',
    key: 'Docker',
    url: 'https://registry-1.docker.io/v2/',
  },
  {
    label: 'Youtube',
    key: 'Youtube',
    url: 'https://www.youtube.com',
  },
  {
    label: 'X',
    key: 'X',
    url: 'https://www.x.com',
  },
  {
    label: 'Twitch',
    key: 'Twitch',
    url: 'https://www.twitch.tv',
  },
])

const speedMenu = vueRef()

async function onShowSpeed() {
  const proxyUrl = `http://127.0.0.1:${settingStore.value.port}`
  speeds.value.forEach((item) => {
    currentProxyDelay(proxyUrl, item.url).then((delay) => {
      item.delay = delay
      item.label = `${delay}ms`
    })
  })
}

function toggleSpeedMenu(event: Event) {
  onShowSpeed()
  speedMenu.value.toggle(event)
}
</script>

<template>
  <div class="flex flex-col w-full h-full gap-y-4">
    <HeaderBar>
      <template #title>
        {{ t('menubar.proxies') }}
      </template>
      <template #default>
        <Button
          rounded
          size="small"
          severity="secondary"
          class="glass-btn"
          @click="showInsertModal = true"
        >
          {{ t('common.add') }}
        </Button>
        <Button
          rounded
          size="small"
          severity="secondary"
          class="glass-btn"
          @click="showImportModal = true"
        >
          {{ t('common.import') }}
        </Button>
      </template>
    </HeaderBar>
    <div class="h-8 flex justify-center items-center gap-2 px-4">
      <SelectButton
        v-model="proxyStore.currentProxy"
        :options="proxyTypeOptions"
        option-label="label"
        option-value="value"
        class="glass-panel"
      />
    </div>
    <div class="flex-1 w-full overflow-y-hidden px-4">
      <ProxyCardList
        :data="cards"
        @dblclick="handleCardDblClick"
      />
    </div>
    <AddProxy
      v-model:show-modal="showInsertModal"
      :current-tab="proxyStore.currentProxy"
      @insert-submit="handleGetAllProxyByType"
    />

    <ImportProxy
      v-model:show-modal="showImportModal"
      :current-tab="ProxyType.Xray"
      :disabled-tab="ProxyType.Hysteria"
      @on-import="handleGetAllProxyByType"
    />
    <EditProxy
      v-model:show-modal="showEditModal"
      :proxy-type="editProxyType"
      :form="(editingProxy as HysteriaProxy | XrayProxy)"
      @on-cancel-edit="handleCancelEdit"
      @on-proxy-updated="handleUpdatedProxy"
    />
    <Button
      v-if="proxyStore.currentProxy === ProxyType.Xray"
      class="!fixed !right-5 !top-[70px] !w-10 !h-10 !rounded-full !p-0 z-50 glass-btn"
      severity="help"
      @click="toggleSpeedMenu"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 20 20"
        class="w-5 h-5"
      >
        <g fill="none">
          <path
            d="M6.19 2.77c.131-.456.548-.77 1.022-.77h5.25c.725 0 1.237.71 1.007 1.398l-.002.008L12.205 7h2.564c.947 0 1.407 1.144.767 1.811l-.004.004l-8.676 8.858c-.755.782-2.06.06-1.796-.996l1.17-4.679H4.963a1.062 1.062 0 0 1-1.022-1.354l2.25-7.873zM7.213 3a.062.062 0 0 0-.06.045l-2.25 7.874c-.01.04.02.08.06.08H6.87a.5.5 0 0 1 .485.62l-1.325 5.3a.086.086 0 0 0-.003.03c0 .004.002.008.003.011c.004.008.013.02.03.03c.018.01.034.01.042.01a.03.03 0 0 0 .01-.004a.087.087 0 0 0 .024-.018l.004-.004l8.675-8.856a.056.056 0 0 0 .017-.032a.084.084 0 0 0-.007-.044a.079.079 0 0 0-.025-.034c-.005-.004-.013-.008-.03-.008H11.5a.5.5 0 0 1-.472-.666l1.493-4.254a.062.062 0 0 0-.06-.08H7.212z"
            fill="currentColor"
            class="text-purple-400"
          />
        </g>
      </svg>
    </Button>
    <Menu
      ref="speedMenu"
      :model="speeds"
      popup
    />
  </div>
</template>

<style lang="scss" scoped>
</style>
