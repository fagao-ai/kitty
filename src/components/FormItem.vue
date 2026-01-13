<script setup lang="ts">
import InputText from 'primevue/inputtext'
import Textarea from 'primevue/textarea'
import Select from 'primevue/select'
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
    <div class="flex flex-col gap-1 mb-3">
      <label class="font-semibold text-sm">{{ formState.payload.label }}</label>

      <InputText
        v-if="formState.payload.type === 'input'"
        v-model="formState.payload.value"
        class="w-full"
      />

      <Textarea
        v-else-if="formState.payload.type === 'textarea'"
        v-model="formState.payload.value"
        class="w-full"
        rows="3"
      />

      <Select
        v-else-if="formState.payload.type === 'select'"
        v-model="formState.payload.value"
        :options="formState.payload.options"
        class="w-full"
      />
    </div>
    <form-item :form="getNext()" />
  </template>
</template>
