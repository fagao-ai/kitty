<template>
  <div
    class="h-16 flex justify-between items-center gap-y-1 p-4"
    data-tauri-drag-region
  >
    <div
      class="text-primay text-2xl font-extrabold cursor-default"
      data-tauri-drag-region
    >
      <slot name="title" />
    </div>
    <div class="flex gap-x-3">
      <slot name="default" />
    </div>
  </div>
</template>
