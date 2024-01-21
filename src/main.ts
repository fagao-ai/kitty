import { createApp } from 'vue'
import naive from 'naive-ui'
import '@/styles.scss'
import App from '@/App.vue'
import router from '@/routers'
import { i18n } from '@/translations'
import 'reflect-metadata'

const app = createApp(App)

app.use(router)
app.use(naive)
app.use(i18n)
app.mount('#app')
