<script setup lang="ts">
import PrimeButton from 'primevue/button'
import PrimeCard from 'primevue/card'
import PrimeDialog from 'primevue/dialog'
import PrimeProgressSpinner from 'primevue/progressspinner'
import { onMounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { emit } from '@tauri-apps/api/event'
import { subscriptionStore } from './store'
import AddSubscription from './modal/AddSubscription.vue'
import EditSubscription from './modal/EditSubscription.vue'
import Empty from '@/components/Empty.vue'
import HeaderBar from '@/components/HeaderBar.vue'
import { deleteSubscription, getAllSubscriptions, refreshSubscription, switchSubscription } from '@/apis/subscription'
import type { SubscriptionInfo } from '@/types/subscription'
import { useMessage } from '@/utils/message'

defineEmits<{
  toggleMobileMenu: []
}>()

const { t } = useI18n()
const message = useMessage()

const showAddModal = ref(false)
const showEditModal = ref(false)
const editingSubscription = ref<SubscriptionInfo | null>(null)
const pendingDeleteSubscription = ref<SubscriptionInfo | null>(null)

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

function requestDelete(subscription: SubscriptionInfo) {
  pendingDeleteSubscription.value = subscription
}

function closeDeleteDialog() {
  pendingDeleteSubscription.value = null
}

function handleDeleteDialogVisibility(visible: boolean) {
  if (!visible) {
    closeDeleteDialog()
  }
}

async function confirmDelete() {
  if (!pendingDeleteSubscription.value)
    return

  const { id } = pendingDeleteSubscription.value
  try {
    await handleDelete(id)
  }
  finally {
    closeDeleteDialog()
  }
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
        <span class="inline-flex h-6 w-6 items-center justify-center">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M3 12h18M3 6h18M3 18h18" stroke-linecap="round" stroke-linejoin="round" />
          </svg>
        </span>
      </template>
      <template #title>
        {{ t('menubar.subscriptions') }}
      </template>
      <template #default>
        <prime-button size="small" :label="t('common.add')" @click="showAddModal = true" />
      </template>
    </header-bar>

    <div class="flex-1 w-full overflow-y-auto px-4">
      <div v-if="isLoading" class="flex h-full items-center justify-center py-10">
        <prime-progress-spinner style="width: 40px; height: 40px" stroke-width="6" />
      </div>

      <div v-else-if="subscriptionStore.subscriptions.length === 0" class="h-full flex items-center justify-center">
        <empty description="No subscriptions yet" />
      </div>

      <div v-else class="grid grid-cols-1 gap-4 pb-4">
        <prime-card
          v-for="sub in subscriptionStore.subscriptions"
          :key="sub.id"
          class="subscription-card"
        >
          <template #header>
            <div class="flex items-center gap-2">
              <span class="h-2.5 w-2.5 rounded-full" :class="sub.isActive ? 'bg-emerald-500' : 'bg-slate-300 dark:bg-slate-600'" />
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
              <prime-button
                v-if="!sub.isActive"
                size="small"
                label="Switch"
                :loading="operatingId === sub.id"
                @click="handleSwitch(sub)"
              />

              <prime-button
                v-if="sub.isActive"
                size="small"
                severity="secondary"
                variant="outlined"
                label="Refresh"
                :loading="operatingId === sub.id"
                @click="handleRefresh(sub.id)"
              />

              <prime-button
                size="small"
                severity="secondary"
                variant="outlined"
                label="Edit"
                @click="handleEdit(sub)"
              />

              <prime-button size="small" severity="danger" variant="outlined" label="Delete" @click="requestDelete(sub)" />
            </div>
          </template>
        </prime-card>
      </div>
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

    <prime-dialog
      :visible="pendingDeleteSubscription !== null"
      modal
      header="Delete Subscription"
      :style="{ width: '28rem', maxWidth: '92vw' }"
      @update:visible="handleDeleteDialogVisibility"
    >
      <p class="m-0 text-sm text-slate-600 dark:text-slate-300">
        Delete subscription and its {{ pendingDeleteSubscription?.nodeCount ?? 0 }} nodes?
      </p>
      <template #footer>
        <div class="flex justify-end gap-3">
          <prime-button label="Cancel" severity="secondary" variant="outlined" @click="closeDeleteDialog" />
          <prime-button
            label="Delete"
            severity="danger"
            :loading="operatingId === pendingDeleteSubscription?.id"
            @click="confirmDelete"
          />
        </div>
      </template>
    </prime-dialog>
  </div>
</template>

<style scoped lang="scss">
:deep(.subscription-card .p-card) {
  border-radius: 12px;
}
</style>
