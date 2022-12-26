import { defineStore } from 'pinia'

export const useGlobalMessagesStore = defineStore('global-messages', () => {
  return useMessages()
})
