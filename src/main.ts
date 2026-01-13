import { createApp } from 'vue'
import PrimeVue from 'primevue/config'
import Aura from '@primevue/themes/aura'
import ConfirmationService from 'primevue/confirmationservice'
import ToastService from 'primevue/toastservice'
import '@/styles.scss'
import App from '@/App.vue'
import router from '@/routers'
import { i18n } from '@/translations'
import 'reflect-metadata'
import 'primeicons/primeicons.css'

const app = createApp(App)

app.use(router)
app.use(PrimeVue, {
  theme: {
    preset: Aura,
    options: {
      darkModeSelector: '.dark',
      prefix: 'p',
      cssLayer: false,
    },
  },
})
app.use(ConfirmationService)
app.use(ToastService)
app.use(i18n)
app.mount('#app')
