<script setup lang="ts">
import { reactive, ref } from 'vue'
import { NButton, NForm, NFormItem, NInput, NModal, useMessage } from 'naive-ui'
import { useVModel } from '@vueuse/core'
import { createSubscription } from '@/apis/subscription'

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
  <n-modal
    v-model:show="showModalRef"
    preset="card"
    title="Add Subscription"
    size="medium"
    :mask-closable="false"
    :bordered="false"
    :segmented="true"
  >
    <n-form
      :model="formState"
      size="medium"
      label-placement="left"
      label-width="100px"
    >
      <n-form-item label="Name" path="name">
        <n-input
          v-model:value="formState.name"
          placeholder="e.g., Primary Provider"
        />
      </n-form-item>

      <n-form-item label="URL" path="url">
        <n-input
          v-model:value="formState.url"
          placeholder="https://example.com/subscription"
          type="textarea"
          :rows="3"
        />
      </n-form-item>
    </n-form>

    <template #footer>
      <div class="w-full flex flex-center gap-3">
        <n-button @click="handleCancel">
          Cancel
        </n-button>
        <n-button
          type="primary"
          :loading="isLoading"
          @click="handleSubmit"
        >
          Add
        </n-button>
      </div>
    </template>
  </n-modal>
</template>
