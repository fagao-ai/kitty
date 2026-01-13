<script setup lang="ts">
import ScrollPanel from 'primevue/scrollpanel'
import ProxyCard from '@/components/ProxyCard.vue'
import Empty from '@/components/Empty.vue'
import type { ProxyCard as Card, ProxyType } from '@/types/proxy'

interface Props {
  data: Card[]
}

interface Emits {
  (e: 'dblclick', id: number, type: ProxyType): void
}

defineProps<Props>()
const emits = defineEmits<Emits>()
</script>

<template>
  <div class="w-full h-full">
    <scroll-panel class="glass-panel" style="width: 100%; height: 100%; border-radius: 0; background: transparent;">
      <template v-if="data.length !== 0">
        <div class="grid grid-cols-5 auto-rows-fr gap-4 p-4 xl:grid-cols-6 xxl:grid-cols-7 xxxl:grid-cols-8 tv:grid-cols-10">
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
              @dblclick="emits('dblclick', card.id, card.type)"
            />
          </template>
        </div>
      </template>
      <empty
        v-else
        description="No Proxy Found"
      />
    </scroll-panel>
  </div>
</template>
