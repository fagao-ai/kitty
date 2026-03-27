<script setup lang="ts">
import PrimeProgressSpinner from 'primevue/progressspinner'
import PrimeTag from 'primevue/tag'
import { computed } from 'vue'
import type { ProxyCard, ProxyType } from '@/types/proxy'

interface Emits {
  (e: 'dblclick', id: number, type: ProxyType): void
  (e: 'click', id: number, type: ProxyType): void
}

const props = defineProps<ProxyCard & { switchingId?: number | null }>()
const emits = defineEmits<Emits>()

const isSwitching = computed(() => props.switchingId === props.id)

const tagType = computed(() => {
  if (props.delay <= 500)
    return 'success'

  if (props.delay <= 1000)
    return 'warn'

  return 'danger'
})

function handleDblClick() {
  emits('dblclick', props.id, props.type)
}

function handleClick() {
  emits('click', props.id, props.type)
}
</script>

<template>
  <div
    class="card-wrapper"
    :class="{ 'active-card': isActive, 'switching-card': isSwitching }"
    role="button"
    :aria-label="`${name} - ${protocol} - ${delay}ms`"
    :aria-pressed="isActive"
    tabindex="0"
    @click="handleClick"
    @dblclick="handleDblClick"
    @keydown.enter="handleClick"
  >
    <div v-if="isSwitching" class="loading-overlay">
      <prime-progress-spinner style="width: 28px; height: 28px" stroke-width="6" />
    </div>

    <div v-if="source === 'subscription'" class="absolute top-2 right-2 z-10">
      <prime-tag severity="warn" value="SUB" />
    </div>

    <div class="h-6">
      <prime-tag :severity="tagType" :value="tag" />
    </div>

    <div class="flex-1 text-sm font-medium truncate text-text-primary dark:text-text-primary">
      {{ name }}
    </div>

    <div class="h-6 flex justify-between items-center text-xs gap-2">
      <span class="text-text-secondary dark:text-text-secondary font-mono">
        {{ delay }}ms
      </span>
      <prime-tag severity="secondary" :value="protocol" />
    </div>
  </div>
</template>

<style scoped lang="scss">
.card-wrapper {
  @apply relative;
  @apply transition-all duration-250 cursor-pointer;
  @apply w-[130px] min-w-[120px] h-[110px] xl:w-[140px] shadow-card bg-bg-card dark:bg-dark-bg-card py-3 px-3 flex flex-col gap-[2px] rounded-lg;
  @apply hover:shadow-card-hover hover:-translate-y-1;
  @apply focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-offset-2;
}

.active-card {
  @apply bg-primary-light/20 dark:bg-primary/10;
  box-shadow: 0 0 0 2px #5352ed, 0 8px 16px rgba(83, 82, 237, 0.15);
}

.switching-card {
  @apply opacity-60 pointer-events-none;
}

.loading-overlay {
  @apply absolute inset-0 z-20;
  @apply flex items-center justify-center;
  @apply bg-bg-card/80 dark:bg-dark-bg-card/80;
  @apply rounded-lg;
}

:deep(.p-tag) {
  @apply text-[11px];
}

.card-wrapper:active {
  @apply scale-[0.98];
}
</style>
