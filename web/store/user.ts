import { defineStore } from 'pinia'
import { useStorage } from '@vueuse/core'
import { UserOutType } from '~/types'

export const useUserStore = defineStore('user', () => {
  const nuxtApp = useNuxtApp()

  const user = ref<UserOutType | null>(null)
  const { execute: getMe } = useGetMe({ key: 'getMe', emitError: false })
  const { execute: changeMeLocale, onSuccess: changeMeLocaleOnSuccess } =
    useChangeMeLocale({
      key: 'changeMeLocale',
    })
  changeMeLocaleOnSuccess((changedUser) => {
    user.value = changedUser
  })
  const updateMe = async () => {
    if (!user.value) {
      user.value = (await getMe()).data
    }
    if (user.value) {
      updateLocale()
    }
  }

  const storageLocale = useStorage('locale', 'en-US')
  const locale = ref('en-US')
  const computedLocale = computed({
    get: () => locale.value,
    set: (value) => {
      locale.value = value
      storageLocale.value = value
      nuxtApp.$i18n.global.locale.value = value
      nuxtApp.$vuetify.locale.current.value = value
      if (user.value && user.value.locale !== value) {
        changeMeLocale(value)
      }
    },
  })
  const updateLocale = () => {
    if (user.value && user.value.locale) {
      computedLocale.value = user.value.locale
    } else {
      computedLocale.value = storageLocale.value
    }
  }

  return {
    user,
    updateMe,
    locale: computedLocale,
    updateLocale,
  }
})
