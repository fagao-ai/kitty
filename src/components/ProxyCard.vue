<script setup lang="ts">
import { computed } from 'vue'
import type { ProxyCard, ProxyType } from '@/types/proxy'

interface Emits {
  (e: 'dblclick', id: number, type: ProxyType): void
}
const props = defineProps<ProxyCard>()
const emits = defineEmits<Emits>()

const delayColorClass = computed(() => {
  if (props.delay <= 500)
    return 'bg-green-400'
  if (props.delay <= 1000)
    return 'bg-yellow-400'
  return 'bg-red-400'
})

const delayTextColor = computed(() => {
  if (props.delay <= 500)
    return 'text-green-400'
  if (props.delay <= 1000)
    return 'text-yellow-400'
  return 'text-red-400'
})

const delayGlowColor = computed(() => {
  if (props.delay <= 500)
    return 'shadow-green-400/30'
  if (props.delay <= 1000)
    return 'shadow-yellow-400/30'
  return 'shadow-red-400/30'
})

async function handleDblClick() {
  emits('dblclick', props.id, props.type)
}
</script>

<template>
  <div
    class="glass-card group relative overflow-hidden transition-all duration-300 hover:scale-105 cursor-pointer w-[140px] h-[120px]"
    @dblclick="handleDblClick"
  >
    <!-- Hover gradient border effect -->
    <div class="absolute inset-0 bg-gradient-to-r from-purple-500 via-pink-500 to-purple-500 opacity-0 group-hover:opacity-100 transition-opacity duration-500 -z-10 blur-sm rounded-lg" />

    <!-- Glass content area -->
    <div class="relative h-full bg-white/10 dark:bg-gray-900/40 backdrop-blur-xl rounded-lg p-3 flex flex-col border border-white/20 dark:border-white/10">
      <!-- Delay indicator (inline) -->
      <div class="flex items-center gap-1.5">
        <div
          class="w-2 h-2 rounded-full animate-pulse"
          :class="delayColorClass"
        />
        <span
          class="text-xs font-semibold"
          :class="delayTextColor"
        >{{ tag }}</span>
      </div>

      <!-- Proxy name -->
      <div class="flex-1 text-sm font-medium text-gray-800 dark:text-gray-100 mt-1 line-clamp-2 overflow-hidden">
        {{ name }}
      </div>

      <!-- Bottom info -->
      <div class="flex justify-between items-center">
        <div class="text-xs font-mono" :class="delayTextColor">
          {{ delay }}ms
        </div>
        <div class="glass-tag px-2 py-0.5 rounded-md bg-purple-500/20 dark:bg-purple-500/15 border border-purple-400/20">
          <span class="text-xs text-purple-600 dark:text-purple-300">{{ protocol }}</span>
        </div>
      </div>
    </div>

    <!-- Glow effect on hover -->
    <div
      class="absolute inset-0 rounded-lg opacity-0 group-hover:opacity-100 transition-opacity duration-300 pointer-events-none"
      :class="delayGlowColor"
      style="box-shadow: inset 0 0 20px currentColor;"
    />
  </div>
</template>
