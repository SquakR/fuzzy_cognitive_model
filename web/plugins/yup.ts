import * as yup from 'yup'
import { useUserStore } from '~/store'

export default defineNuxtPlugin(() => {
  const userStore = useUserStore()

  return {
    provide: {
      yup: {
        ...yup,
        number: () => {
          if (userStore.locale === 'ru-RU') {
            return yup.number().transform((_, v) => {
              if (v.includes('.')) {
                return NaN
              }
              return parseFloat(v.replace(',', '.'))
            })
          }
          return yup.number()
        },
      },
    },
  }
})
