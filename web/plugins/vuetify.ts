import { createVuetify } from 'vuetify'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'
import colors from 'vuetify/lib/util/colors'
import { en, ru } from 'vuetify/locale'

export default defineNuxtPlugin((nuxtApp) => {
  const vuetify = createVuetify({
    ssr: true,
    theme: {
      themes: {
        light: {
          dark: false,
          colors: {
            primary: colors.indigo.base,
          },
        },
      },
    },
    locale: {
      locale: 'en-US',
      messages: { 'en-US': en, 'ru-RU': ru },
    },
    components,
    directives,
  })

  nuxtApp.vueApp.use(vuetify)

  return {
    provide: {
      vuetify,
    },
  }
})
