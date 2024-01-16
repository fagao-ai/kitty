<script setup lang="ts">
interface Props {
  description?: string
}

withDefaults(defineProps<Props>(), {
  description: 'No Data',
})
</script>

<template>
  <div class="w-full flex justify-center items-center">
    <n-empty
      size="huge"
      :description="description"
    />
  </div>
</template>
