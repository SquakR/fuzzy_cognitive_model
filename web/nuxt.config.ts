// https://nuxt.com/docs/api/configuration/nuxt-config
import VueI18nPlugin from '@intlify/unplugin-vue-i18n'

export default defineNuxtConfig({
  css: [
    '@fortawesome/fontawesome-svg-core/styles.css',
    'assets/styles/main.sass',
  ],
  vite: {
    plugins: [VueI18nPlugin.vite({})],
  },
  runtimeConfig: {
    public: {
      API_HTTP_BASE_URL: process.env.API_HTTP_BASE_URL,
    },
  },
  modules: ['@vueuse/nuxt'],
})
