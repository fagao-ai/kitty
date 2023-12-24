import { createI18n } from 'vue-i18n'

// import { zh } from '@/translations/zh'
// import { en } from '@/translations/en'
import zhCN from '@/translations/zh-CN.json'
import enUS from '@/translations/en-US.json'

type MessageSchema = typeof zhCN

const messages = {
  'zh-CN': zhCN,
  'en-US': enUS,
}

export const i18n = createI18n<[MessageSchema], 'zh-CN' | 'en-US'>({
  legacy: false,
  locale: 'zh-CN',
  messages,
})
