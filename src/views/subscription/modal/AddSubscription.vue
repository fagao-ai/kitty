<script setup lang="ts">
import PrimeButton from 'primevue/button'
import PrimeDialog from 'primevue/dialog'
import PrimeInputText from 'primevue/inputtext'
import PrimeTextarea from 'primevue/textarea'
import { reactive, ref } from 'vue'
import { useVModel } from '@vueuse/core'
import { createSubscription } from '@/apis/subscription'
import { useMessage } from '@/utils/message'

interface Props {
  showModal: boolean
}

interface Emits {
  (e: 'update:showModal', value: boolean): void
  (e: 'onAddSuccess'): void
}

const props = withDefaults(defineProps<Props>(), { showModal: false })
const emits = defineEmits<Emits>()

const message = useMessage()
const showModalRef = useVModel(props, 'showModal', emits)

const formState = reactive({
  name: '',
  url: '',
})

const isLoading = ref(false)

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
    await createSubscription(formState.name, formState.url)
    message.success('Subscription created successfully')
    formState.name = ''
    formState.url = ''
    showModalRef.value = false
    emits('onAddSuccess')
  }
  catch (e: any) {
    message.error(e?.message || 'Failed to create subscription')
  }
  finally {
    isLoading.value = false
  }
}

function handleCancel() {
  formState.name = ''
  formState.url = ''
  showModalRef.value = false
}
</script>

<template>
  <prime-dialog
    v-model:visible="showModalRef"
    modal
    header="Add Subscription"
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
          label="Add"
          :loading="isLoading"
          @click="handleSubmit"
        />
      </div>
    </template>
  </prime-dialog>
</template>
