<script
  setup
  lang="ts"
>
import { NButton, useMessage } from 'naive-ui'
import { computed, onUnmounted, ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { ProxyType } from '@/types/proxy'
import AddProxy from '@/views/proxy/modal/AddProxy.vue'
import type { ProxyCard as Card, HysteriaProxy, XrayProxy } from '@/types/proxy'
import { proxyStore } from '@/views/proxy/store'
import ProxyCardList from '@/components/ProxyCardList.vue'
import { getAllHysterias, getAllXraies, getProxyByIdAndType } from '@/apis/proxy'
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

const unwatchProxyStore = watch(proxyStore, () => {
  handleGetAllProxyByType(proxyStore.value.currentProxy)
}, { immediate: true, deep: true })

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

const unwatchUpdateStatus = watch(updateStatus, (newStatus, oldStatus) => {
  if (oldStatus === 'running' && newStatus === 'stop')
    handleGetAllProxyByType(ProxyType.Xray)
})

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
          add
        </n-button>
        <n-button
          round
          size="small"
          @click="showImportModal = true"
        >
          import
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
  </div>
</template>

<style
  lang="scss"
  scoped
>
:deep(.n-radio-button) {
  --n-button-border-radius: 12px;

  .n-radio__label {
    @apply flex items-center justify-center;
  }
}
</style>
