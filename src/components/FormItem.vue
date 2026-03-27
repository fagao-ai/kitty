<script setup lang="ts">
import PrimeInputText from 'primevue/inputtext'
import PrimeSelect from 'primevue/select'
import PrimeTextarea from 'primevue/textarea'
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
    <div class="flex flex-col gap-2">
      <span class="text-sm font-medium">{{ formState.payload.label }}</span>
      <template v-if="formState.payload.type === 'input'">
        <prime-input-text v-model="formState.payload.value" />
      </template>
      <template v-else-if="formState.payload.type === 'textarea'">
        <prime-textarea v-model="formState.payload.value" />
      </template>
      <template v-else-if="formState.payload.type === 'select'">
        <prime-select
          v-model="formState.payload.value"
          :options="formState.payload.options"
          option-label="label"
          option-value="value"
        />
      </template>
    </div>
    <form-item :form="getNext()" />
  </template>
</template>
