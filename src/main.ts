import { createApp } from 'vue'
import naive from 'naive-ui'
import '@/styles.scss'
import App from '@/App.vue'
import router from '@/routers'

const app = createApp(App)

app.use(router)
app.use(naive)
app.mount('#app')
