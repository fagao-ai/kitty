<script setup lang="ts">
import { computed } from 'vue'
import { NTag } from 'naive-ui'
import type { ProxyCard, ProxyType } from '@/types/proxy'

interface Emits {
  (e: 'dblclick', id: number, type: ProxyType): void
  (e: 'click', id: number, type: ProxyType): void
}

const props = defineProps<ProxyCard>()
const emits = defineEmits<Emits>()

const tagType = computed(() => {
  if (props.delay <= 500)
    return 'success'

  if (props.delay <= 1000)
    return 'warning'

  return 'error'
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
    :class="{ 'active-card': isActive }"
    role="button"
    :aria-label="`${name} - ${protocol} - ${delay}ms`"
    :aria-pressed="isActive"
    tabindex="0"
    @click="handleClick"
    @dblclick="handleDblClick"
    @keydown.enter="handleClick"
  >
    <!-- Subscription tag (top right) - only show for subscription nodes -->
    <div v-if="source === 'subscription'" class="absolute top-2 right-2 z-10">
      <n-tag
        size="small"
        type="warning"
        :bordered="false"
      >
        SUB
      </n-tag>
    </div>

    <!-- Delay status tag (top left) -->
    <div class="h-6">
      <n-tag
        :type="tagType"
        size="small"
        round
      >
        {{ tag }}
      </n-tag>
    </div>

    <!-- Node name -->
    <div class="flex-1 text-sm font-medium truncate text-text-primary dark:text-text-primary">
      {{ name }}
    </div>

    <!-- Delay and transport protocol -->
    <div class="h-6 flex justify-between items-center text-xs">
      <span class="text-text-secondary dark:text-text-secondary font-mono">
        {{ delay }}ms
      </span>
      <n-tag
        size="small"
        :bordered="false"
      >
        {{ protocol }}
      </n-tag>
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

// Add active state for keyboard users
.card-wrapper:active {
  @apply scale-[0.98];
}
</style>
