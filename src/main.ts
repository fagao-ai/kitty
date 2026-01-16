import { createApp } from 'vue'
import PrimeVue from 'primevue/config'
import { definePreset } from '@primevue/themes'
import Aura from '@primevue/themes/aura'
import Ripple from 'primevue/ripple'
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

// Create custom purple theme preset
const PurplePreset = definePreset(Aura, {
  semantic: {
    primary: {
      50: '#f3e8ff',
      100: '#e9d5ff',
      200: '#d8b4fe',
      300: '#c084fc',
      400: '#a855f7',
      500: '#8B5CF6',
      600: '#7c3aed',
      700: '#6d28d9',
      800: '#5b21b6',
      900: '#4c1d95',
      950: '#2e1065',
    },
    colorScheme: {
      light: {
        primary: {
          color: '{primary.600}',
          inverseColor: '#ffffff',
          hoverColor: '{primary.700}',
          activeColor: '{primary.800}',
          focusRing: {
            width: '2px',
            style: 'solid',
            color: '{primary.500}',
            offset: '2px',
          },
        },
        highlight: {
          background: '{primary.50}',
          focusBackground: '{primary.100}',
          color: '{primary.700}',
          focusColor: '{primary.800}',
        },
        surface: {
          0: '#ffffff',
          50: '#fafafa',
          100: '#f4f4f5',
          200: '#e4e4e7',
          300: '#d4d4d8',
          400: '#a1a1aa',
          500: '#71717a',
          600: '#52525b',
          700: '#3f3f46',
          800: '#27272a',
          900: '#18181b',
          950: '#09090b',
        },
      },
      dark: {
        primary: {
          color: '{primary.400}',
          inverseColor: '#ffffff',
          hoverColor: '{primary.300}',
          activeColor: '{primary.200}',
          focusRing: {
            width: '2px',
            style: 'solid',
            color: '{primary.400}',
            offset: '2px',
          },
        },
        highlight: {
          background: 'rgba(139, 92, 246, 0.16)',
          focusBackground: 'rgba(139, 92, 246, 0.24)',
          color: 'rgba(255, 255, 255, 0.87)',
          focusColor: 'rgba(255, 255, 255, 0.87)',
        },
        surface: {
          0: '#ffffff',
          50: '#1c1c1e',
          100: '#18181b',
          200: '#09090b',
          300: '#27272a',
          400: '#3f3f46',
          500: '#52525b',
          600: '#71717a',
          700: '#a1a1aa',
          800: '#d4d4d8',
          900: '#e4e4e7',
          950: '#f4f4f5',
        },
      },
    },
  },
})

app.use(PrimeVue, {
  ripple: true,
  theme: {
    preset: PurplePreset,
    options: {
      darkModeSelector: '.dark',
      prefix: 'p',
      cssLayer: false,
    },
  },
})
app.directive('ripple', Ripple)
app.use(ConfirmationService)
app.use(ToastService)
app.use(i18n)
app.mount('#app')
