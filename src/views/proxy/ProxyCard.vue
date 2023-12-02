<template>
	<div class="w-[130px] h-[110px] shadow-2xl bg-white">
		tag: {{ tag }}<br />
		name: {{ name }}<br />
		delay: {{ delay }}<br />
		protocol: {{ protocol }} <br />
	</div>
</template>

<script setup lang="ts">
import type { ProxyCard } from '@/types/proxy'

defineProps<ProxyCard>()
</script>