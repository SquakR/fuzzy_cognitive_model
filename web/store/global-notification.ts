import { defineStore } from 'pinia'

export const useGlobalNotificationStore = defineStore(
  'global-notification',
  () => {
    return useNotification()
  }
)
