import { defineStore } from 'pinia'

export interface Message {
  type: 'success' | 'error'
  message: string
}

export const useMessageStore = defineStore('message', () => {
  const target = ref<'global' | 'local'>('global')
  const message = ref<Message | null>(null)

  return { target, message }
})
