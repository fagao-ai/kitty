<template>
  <div
    class="glass-card h-16 flex justify-between items-center gap-y-1 p-4 mx-4 mt-4 rounded-xl"
    data-tauri-drag-region
  >
    <div
      class="text-gradient-purple text-2xl font-extrabold cursor-default"
      data-tauri-drag-region
    >
      <slot name="title" />
    </div>
    <div class="flex gap-x-3">
      <slot name="default" />
    </div>
  </div>
</template>
