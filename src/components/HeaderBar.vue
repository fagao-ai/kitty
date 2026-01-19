<script setup lang="ts">
interface Props {
  isMobileMenuOpen?: boolean
}

interface Emits {
  (e: 'toggle-mobile-menu'): void
}

defineProps<Props>()
defineEmits<Emits>()
</script>

<template>
  <div
    class="h-14 md:h-16 flex justify-between items-center gap-y-1 px-4 md:px-6"
    data-tauri-drag-region
  >
    <!-- Mobile menu button -->
    <button
      v-if="$slots['mobile-menu-button']"
      class="md:hidden p-2 -ml-2 text-text-primary dark:text-text-primary"
      @click="$emit('toggle-mobile-menu')"
      aria-label="Toggle menu"
    >
      <slot name="mobile-menu-button" />
    </button>

    <div
      class="text-text-primary dark:text-text-primary text-xl md:text-2xl font-bold cursor-default"
      data-tauri-drag-region
    >
      <slot name="title" />
    </div>

    <!-- Desktop actions -->
    <div class="hidden md:flex gap-x-3">
      <slot name="default" />
    </div>

    <!-- Mobile actions (optional) -->
    <div v-if="$slots['mobile-actions']" class="flex md:hidden gap-x-2">
      <slot name="mobile-actions" />
    </div>
  </div>
</template>
