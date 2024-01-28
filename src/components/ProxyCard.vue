<script setup lang="ts">
import { computed } from 'vue'
import { NTag } from 'naive-ui'
import type { ProxyCard, ProxyType } from '@/types/proxy'

interface Emits {
  (e: 'dblclick', id: number, type: ProxyType): void
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

async function handleDblClick() {
  emits('dblclick', props.id, props.type)
}
</script>

<template>
  <div
    class="w-[130px] h-[110px] shadow-2xl bg-[#f9f7f7] py-3 px-2 flex flex-col gap-[2px] rounded-md dark:bg-[#3e4247] dark:text-slate-100"
    @dblclick="handleDblClick"
  >
    >
    <div class="h-6">
      <n-tag
        :type="tagType"
        size="small"
        round
      >
        {{ tag }}
      </n-tag>
    </div>
    <div class="flex-1 text-sm text-[#54759a] dark:text-slate-200">
      {{ name }}
    </div>
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
