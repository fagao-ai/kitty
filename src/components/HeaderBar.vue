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
    class="h-14 md:h-16 flex items-center gap-y-1 px-4 md:px-6 md:justify-between relative"
    data-tauri-drag-region
  >
    <!-- Left side: mobile menu button -->
    <div class="w-12 md:hidden">
      <button
        v-if="$slots['mobile-menu-button']"
        class="p-2 -ml-2 text-text-primary dark:text-text-primary"
        @click="$emit('toggle-mobile-menu')"
        aria-label="Toggle menu"
      >
        <slot name="mobile-menu-button" />
      </button>
    </div>

    <!-- Title - centered on mobile -->
    <div
      class="absolute left-1/2 -translate-x-1/2 md:static md:translate-x-0 md:flex-1 text-center md:text-left text-text-primary dark:text-text-primary text-xl md:text-2xl font-bold cursor-default"
      data-tauri-drag-region
    >
      <slot name="title" />
    </div>

    <!-- Right side: mobile actions or placeholder -->
    <div class="w-12 flex justify-end md:hidden">
      <div v-if="$slots['mobile-actions']" class="flex gap-x-2">
        <slot name="mobile-actions" />
      </div>
    </div>

    <!-- Desktop actions -->
    <div class="hidden md:flex gap-x-3">
      <slot name="default" />
    </div>
  </div>
</template>
