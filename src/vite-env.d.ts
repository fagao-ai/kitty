/* eslint-disable */
/// <reference types="vite/client" />
// import type { MessageApiInjection } from "naive-ui/lib/message/src/MessageProvider"

declare module '*.vue' {
  import type { DefineComponent } from 'vue'
  const component: DefineComponent<{}, {}, any>
  export default component
}

declare module '@tauri-apps/api/primitives' {
  import { transformCallback, Channel, PluginListener, addPluginListener, invoke, convertFileSrc } from '@tauri-apps/api/types/primitives'
  export { transformCallback, Channel, PluginListener, addPluginListener, invoke, convertFileSrc }
}

// declare global {
//   interface Window {
//       $message: MessageApiInjection
//   }
// }