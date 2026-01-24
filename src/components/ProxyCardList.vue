<script
  setup
  lang="ts"
>
import ProxyCard from '@/components/ProxyCard.vue'
import Empty from '@/components/Empty.vue'
import type { ProxyCard as Card, ProxyType } from '@/types/proxy'

interface Props {
  data: Card[]
  switchingId?: number | null
}

interface Emits {
  (e: 'dblclick', id: number, type: ProxyType): void
  (e: 'click', id: number, type: ProxyType): void
}

defineProps<Props>()
const emits = defineEmits<Emits>()
</script>

<template>
  <div class="w-full h-full">
    <n-scrollbar>
      <template v-if="data.length !== 0">
        <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 xxl:grid-cols-7 xxxl:grid-cols-8 tv:grid-cols-10 gap-2">
          <template
            v-for="(card, index) in data"
            :key="index"
          >
            <proxy-card
              :id="card.id"
              :type="card.type"
              :name="card.name"
              :delay="card.delay"
              :tag="card.tag"
              :protocol="card.protocol"
              :source="card.source"
              :is-active="card.isActive"
              :switching-id="switchingId"
              @dblclick="emits('dblclick', card.id, card.type)"
              @click="emits('click', card.id, card.type)"
            />
          </template>
        </div>
      </template>
      <empty
        v-else
        description="No Proxy Found"
      />
    </n-scrollbar>
  </div>
</template>

<style
  lang="scss"
  scoped
>
:deep(.n-scrollbar) {
  .n-scrollbar-content {
    @apply pl-4 py-4;
  }
}
</style>
