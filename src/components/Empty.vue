<script setup lang="ts">
interface Props {
  description?: string
}

withDefaults(defineProps<Props>(), {
  description: 'No Data',
})
</script>

<template>
  <div class="w-full h-full flex flex-col items-center justify-center p-8">
    <div class="glass-panel p-8 rounded-2xl flex flex-col items-center animate-scale-in">
      <div class="w-16 h-16 rounded-full bg-purple-500/20 dark:bg-purple-500/10 flex items-center justify-center mb-4">
        <i class="pi pi-inbox text-3xl text-purple-400" />
      </div>
      <p class="text-gray-500 dark:text-gray-400 text-sm">
        {{ description }}
      </p>
    </div>
  </div>
</template>
