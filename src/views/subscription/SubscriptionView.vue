<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { NBadge, NButton, NCard, NEmpty, NIcon, NPopconfirm, NSpin, useMessage } from 'naive-ui'
import { useI18n } from 'vue-i18n'
import { emit } from '@tauri-apps/api/event'
import { subscriptionStore } from './store'
import AddSubscription from './modal/AddSubscription.vue'
import EditSubscription from './modal/EditSubscription.vue'
import HeaderBar from '@/components/HeaderBar.vue'
import { deleteSubscription, getAllSubscriptions, refreshSubscription, switchSubscription } from '@/apis/subscription'
import type { SubscriptionInfo } from '@/types/subscription'

defineEmits<{
  toggleMobileMenu: []
}>()

const { t } = useI18n()
const message = useMessage()

const showAddModal = ref(false)
const showEditModal = ref(false)
const editingSubscription = ref<SubscriptionInfo | null>(null)

const isLoading = ref(false)
const operatingId = ref<number | null>(null)

// Fetch subscriptions
async function loadSubscriptions() {
  isLoading.value = true
  try {
    const data = await getAllSubscriptions()
    subscriptionStore.value.subscriptions = data
  }
  catch (e: any) {
    message.error(e?.message || 'Failed to load subscriptions')
  }
  finally {
    isLoading.value = false
  }
}

// Delete subscription
async function handleDelete(id: number) {
  operatingId.value = id
  try {
    await deleteSubscription(id)
    message.success('Subscription deleted')
    await loadSubscriptions()
    // Notify proxy page to refresh
    await emit('subscription-changed', { action: 'delete', id })
  }
  catch (e: any) {
    message.error(e?.message || 'Failed to delete subscription')
  }
  finally {
    operatingId.value = null
  }
}

// Switch subscription
async function handleSwitch(subscription: SubscriptionInfo) {
  operatingId.value = subscription.id
  try {
    await switchSubscription(subscription.id)
    message.success('Subscription switched')
    await loadSubscriptions()
    // Notify proxy page to refresh
    await emit('subscription-changed', { action: 'switch', id: subscription.id })
  }
  catch (e: any) {
    message.error(e?.message || 'Failed to switch subscription')
  }
  finally {
    operatingId.value = null
  }
}

// Refresh subscription
async function handleRefresh(id: number) {
  operatingId.value = id
  try {
    await refreshSubscription(id)
    message.success('Subscription refreshed')
    await loadSubscriptions()
    // Notify proxy page to refresh
    await emit('subscription-changed', { action: 'refresh', id })
  }
  catch (e: any) {
    message.error(e?.message || 'Failed to refresh subscription')
  }
  finally {
    operatingId.value = null
  }
}

// Edit subscription
function handleEdit(subscription: SubscriptionInfo) {
  editingSubscription.value = subscription
  showEditModal.value = true
}

// Handle add success
async function handleAddSuccess() {
  showAddModal.value = false
  await loadSubscriptions()
  // Notify proxy page to refresh
  await emit('subscription-changed', { action: 'add' })
}

// Handle edit success
async function handleEditSuccess() {
  showEditModal.value = false
  editingSubscription.value = null
  await loadSubscriptions()
  // Notify proxy page to refresh (edit doesn't affect proxy nodes, but update list anyway)
  await emit('subscription-changed', { action: 'edit' })
}

// Format date
function formatDate(dateString?: string) {
  if (!dateString)
    return '-'
  const date = new Date(dateString)
  return date.toLocaleString()
}

onMounted(() => {
  loadSubscriptions()
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
        {{ t('menubar.subscriptions') }}
      </template>
      <template #default>
        <n-button
          size="small"
          @click="showAddModal = true"
        >
          {{ t('common.add') }}
        </n-button>
      </template>
    </header-bar>

    <div class="flex-1 w-full overflow-y-auto px-4">
      <n-spin :show="isLoading">
        <div v-if="subscriptionStore.subscriptions.length === 0 && !isLoading" class="h-full flex items-center justify-center">
          <n-empty description="No subscriptions yet" />
        </div>

        <div v-else class="grid grid-cols-1 gap-4 pb-4">
          <n-card
            v-for="sub in subscriptionStore.subscriptions"
            :key="sub.id"
            :bordered="false"
            size="small"
            class="hover:shadow-md transition-shadow"
          >
            <template #header>
              <div class="flex items-center gap-2">
                <n-badge
                  :type="sub.isActive ? 'success' : 'default'"
                  :dot="sub.isActive"
                />
                <span class="font-medium">{{ sub.name }}</span>
                <span v-if="sub.isActive" class="text-xs text-success ml-auto">(Active)</span>
              </div>
            </template>

            <div class="space-y-2">
              <div class="text-sm text-gray-600 dark:text-gray-400 break-all">
                {{ sub.url }}
              </div>
              <div class="text-xs text-gray-500 dark:text-gray-500">
                Nodes: {{ sub.nodeCount }} | Updated: {{ formatDate(sub.updatedAt) }}
              </div>
            </div>

            <template #footer>
              <div class="flex gap-2 justify-end">
                <n-button
                  v-if="!sub.isActive"
                  size="small"
                  type="primary"
                  :loading="operatingId === sub.id"
                  @click="handleSwitch(sub)"
                >
                  Switch
                </n-button>

                <n-button
                  v-if="sub.isActive"
                  size="small"
                  :loading="operatingId === sub.id"
                  @click="handleRefresh(sub.id)"
                >
                  Refresh
                </n-button>

                <n-button
                  size="small"
                  @click="handleEdit(sub)"
                >
                  Edit
                </n-button>

                <n-popconfirm
                  @positive-click="handleDelete(sub.id)"
                >
                  <template #trigger>
                    <n-button size="small" type="error">
                      Delete
                    </n-button>
                  </template>
                  Delete subscription and its {{ sub.nodeCount }} nodes?
                </n-popconfirm>
              </div>
            </template>
          </n-card>
        </div>
      </n-spin>
    </div>

    <add-subscription
      v-model:show-modal="showAddModal"
      @on-add-success="handleAddSuccess"
    />

    <edit-subscription
      v-if="editingSubscription"
      v-model:show-modal="showEditModal"
      :subscription="editingSubscription"
      @on-edit-success="handleEditSuccess"
    />
  </div>
</template>

<style scoped lang="scss">
:deep(.n-card) {
  border-radius: 8px;
}
</style>
