import { defineStore } from 'pinia'
import { useStorage } from '@vueuse/core'
import { useI18n } from 'vue-i18n'

export const useLocaleStore = defineStore('locale', () => {
  const storageLocale = useStorage('locale', 'en-US')
  const { locale: i18nLocale } = useI18n({ useScope: 'global' })
  const locale = ref('en-US')
  const computedLocale = computed({
    get: () => locale.value,
    set: (value) => {
      locale.value = value
      storageLocale.value = value
      i18nLocale.value = value
    },
  })
  onMounted(() => {
    computedLocale.value = storageLocale.value
  })
  return {
    locale: computedLocale,
  }
})
