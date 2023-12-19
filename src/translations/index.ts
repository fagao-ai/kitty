import { createI18n } from 'vue-i18n'
import { zh } from '@/translations/zh'
import { en } from '@/translations/en'

const messages = {
  zh,
  en,
}

export const i18n = createI18n({
  legacy: false,
  locale: 'zh',
  messages,
})
