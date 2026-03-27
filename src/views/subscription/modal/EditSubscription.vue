<script setup lang="ts">
import PrimeButton from 'primevue/button'
import PrimeDialog from 'primevue/dialog'
import PrimeInputText from 'primevue/inputtext'
import PrimeTextarea from 'primevue/textarea'
import { reactive, ref, watch } from 'vue'
import { useVModel } from '@vueuse/core'
import { updateSubscription } from '@/apis/subscription'
import type { SubscriptionInfo } from '@/types/subscription'
import { useMessage } from '@/utils/message'

interface Props {
  showModal: boolean
  subscription: SubscriptionInfo
}

interface Emits {
  (e: 'update:showModal', value: boolean): void
  (e: 'onEditSuccess'): void
}

const props = defineProps<Props>()
const emits = defineEmits<Emits>()

const message = useMessage()
const showModalRef = useVModel(props, 'showModal', emits)

const formState = reactive({
  name: '',
  url: '',
})

const isLoading = ref(false)

watch(() => props.subscription, (newVal) => {
  if (newVal) {
    formState.name = newVal.name
    formState.url = newVal.url
  }
}, { immediate: true })

async function handleSubmit() {
  if (!formState.name.trim()) {
    message.error('Please enter subscription name')
    return
  }
  if (!formState.url.trim()) {
    message.error('Please enter subscription URL')
    return
  }

  isLoading.value = true
  try {
    await updateSubscription(props.subscription.id, formState.name, formState.url)
    message.success('Subscription updated successfully')
    showModalRef.value = false
    emits('onEditSuccess')
  }
  catch (e: any) {
    message.error(e?.message || 'Failed to update subscription')
  }
  finally {
    isLoading.value = false
  }
}

function handleCancel() {
  showModalRef.value = false
}
</script>

<template>
  <prime-dialog
    v-model:visible="showModalRef"
    modal
    header="Edit Subscription"
    :style="{ width: '32rem', maxWidth: '92vw' }"
  >
    <div class="flex flex-col gap-5">
      <label class="flex flex-col gap-2">
        <span class="text-sm font-medium text-text-primary dark:text-text-primary">Name</span>
        <prime-input-text
          v-model="formState.name"
          placeholder="e.g., Primary Provider"
        />
      </label>

      <label class="flex flex-col gap-2">
        <span class="text-sm font-medium text-text-primary dark:text-text-primary">URL</span>
        <prime-textarea
          v-model="formState.url"
          placeholder="https://example.com/subscription"
          :rows="3"
        />
      </label>
    </div>

    <template #footer>
      <div class="w-full flex flex-center gap-3">
        <prime-button label="Cancel" severity="secondary" variant="outlined" @click="handleCancel" />
        <prime-button
          label="Update"
          :loading="isLoading"
          @click="handleSubmit"
        />
      </div>
    </template>
  </prime-dialog>
</template>
