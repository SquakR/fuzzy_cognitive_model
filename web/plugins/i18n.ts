import { createI18n } from 'vue-i18n'

export default defineNuxtPlugin(({ vueApp }) => {
  const i18n = createI18n({
    legacy: false,
    globalInjection: false,
    locale: 'en-US',
    availableLocales: ['en-US', 'ru-RU'],
  })

  vueApp.use(i18n)
  return {
    provide: {
      i18n,
    },
  }
})
