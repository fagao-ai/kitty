<script setup lang="ts">
import { NFormItem, NInput, NSelect } from 'naive-ui'
import { useVModel } from '@vueuse/core'
import { FormItem } from '@/composables/useFormItem'

interface Props {
  form: FormItem | null
}
const props = defineProps<Props>()

const formState = useVModel(props, 'form')

function getNext(): FormItem | null {
  let current = formState.value
  if (!current)
    return null

  const acients = []
  acients.unshift(current)

  while (current) {
    acients.unshift(current)
    current = current.parent
  }

  return formState.value!.next(formState.value!, acients)
}
</script>

<template>
  <template v-if="formState">
    <n-form-item :label="formState.payload.label">
      <template v-if="formState.payload.type === 'input'">
        <n-input v-model:value="formState.payload.value" />
      </template>
      <template v-else-if="formState.payload.type === 'textarea'">
        <n-input
          v-model:value="formState.payload.value"
          type="textarea"
        />
      </template>
      <template v-else-if="formState.payload.type === 'select'">
        <n-select
          v-model:value="formState.payload.value"
          :options="formState.payload.options"
        />
      </template>
  </n-form-item>
  <form-item :form-state="getNext()" />
</template></template>
