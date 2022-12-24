import { createI18n } from 'vue-i18n'

export default defineNuxtPlugin(({ vueApp }) => {
  const i18n = createI18n({
    legacy: false,
    globalInjection: false,
    locale: 'en-ES',
    availableLocales: ['en-ES', 'ru-RU'],
  })

  vueApp.use(i18n)
})
