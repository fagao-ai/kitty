<script setup lang="ts">
interface Props {
  description?: string
}

withDefaults(defineProps<Props>(), {
  description: 'No Data',
})
</script>

<template>
  <div class="w-full flex flex-col items-center justify-center p-8 text-surface-500">
    <i class="pi pi-inbox text-4xl mb-4" />
    <p>{{ description }}</p>
  </div>
</template>
