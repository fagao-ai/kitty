<script setup lang="ts">
import PrimeMessage from 'primevue/message'

interface Props {
  description?: string
}

withDefaults(defineProps<Props>(), {
  description: 'No Data',
})
</script>

<template>
  <div class="w-full flex justify-center items-center py-8">
    <prime-message severity="secondary" class="!rounded-xl !px-4 !py-3">
      {{ description }}
    </prime-message>
  </div>
</template>
