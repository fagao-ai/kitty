<script setup lang="ts">
import { h } from 'vue'
import { NMenu } from 'naive-ui'
import type { MenuOption } from 'naive-ui'
import { RouterLink } from 'vue-router'

const menuOptions: MenuOption[] = [
  {
    label: () =>
      h(
        RouterLink,
        {
          to: {
            name: 'proxy',
          },
        },
        { default: () => 'Proxies' },
      ),
    key: 'proxy',
  },
  {
    label: () =>
      h(
        RouterLink,
        {
          to: {
            name: 'setting',
          },
        },
        { default: () => 'Settings' },
      ),
    key: 'setting',
  },
]
</script>

<template>
  <div class="flex flex-col h-full p-4">
    <div class="flex-1 flex flex-col">
      <div class="text-5xl font-bold h-1/6 flex flex-center text-primay">
        kitty
      </div>
      <div class="flex-1 text-white text-lg">
        <n-menu
          default-value="proxy"
          :options="menuOptions"
        />
      </div>
    </div>
    <div class="h-1/8 flex flex-center flex-col">
      <div class="text-primay text-lg">
        version
      </div>
      <div class="text-primay">
        0.0.1.beta
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
@keyframes roundAnimation {
  0% {
    border-radius: 0;
  }

  100% {
    border-radius: 999px;
  }
}

:deep(.n-menu) {
  --n-item-color-active: #5352ed !important;
  --n-item-color-active-hover: #5352ed !important;

  .n-menu-item-content--selected {
    &::before {
      animation-name: roundAnimation;
      animation-duration: 15s;
      animation-fill-mode: forwards;
    }
  }

  .n-menu-item-content {
    @apply flex justify-center items-center;
    padding-left: 0 !important;
    padding-right: 0 !important;
  }

  .router-link-active {
    font-size: 18px;
    color: white !important;
  }
}
</style>
