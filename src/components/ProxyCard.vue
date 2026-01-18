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
    @click="handleClick"
    @dblclick="handleDblClick"
  >
    <!-- Subscription tag (top right) - only show for subscription nodes -->
    <div v-if="source === 'subscription'" class="absolute top-2 right-2">
      <n-tag
        size="small"
        type="warning"
        :bordered="false"
      >
        SUB
      </n-tag>
    </div>

    <!-- Original tag (top left, kept for delay status) -->
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
    <div class="flex-1 text-sm text-[#54759a] dark:text-slate-200">
      {{ name }}
    </div>

    <!-- Delay and transport protocol -->
    <div class="h-6 flex justify-between items-center">
      <div>
        {{ delay }}ms
      </div>
      <n-tag
        round
        size="small"
      >
        {{ protocol }}
      </n-tag>
    </div>
  </div>
</template>

<style scoped lang="scss">
.card-wrapper {
  @apply relative;
  @apply transform transition-transform duration-500 hover:scale-110 cursor-pointer;
  @apply w-[130px] h-[110px] shadow-2xl bg-[#f9f7f7] py-3 px-2 flex flex-col gap-[2px] rounded-md;
  @apply dark:bg-[#3e4247] dark:text-slate-100;
}

.active-card {
  @apply bg-[#e6fffa] dark:bg-[#2d3748];
  box-shadow: 0 0 15px rgba(99, 226, 183, 0.6);
  position: relative;

  &::before {
    content: '';
    position: absolute;
    inset: 0;
    border-radius: 6px;
    padding: 2px;
    background: linear-gradient(90deg, #00ff88, #00d4ff, #ff00ff, #00ff88, #00d4ff, #ff00ff);
    background-size: 250% 100%;
    -webkit-mask: linear-gradient(#fff 0 0) content-box, linear-gradient(#fff 0 0);
    -webkit-mask-composite: xor;
    mask-composite: exclude;
    animation: borderFlow 2s linear infinite;
    pointer-events: none;
  }
}

@keyframes borderFlow {
  0% {
    background-position: 0% 50%;
  }
  100% {
    background-position: 250% 50%;
  }
}
</style>
