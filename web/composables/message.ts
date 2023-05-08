import { useMessageStore } from '~/store'

export const useLocaleMessage = () => {
  const messageStore = useMessageStore()
  messageStore.target = 'local'
  onUnmounted(() => {
    messageStore.target = 'global'
  })
  return computed({
    get: () => messageStore.message,
    set: (value) => {
      messageStore.message = value
    },
  })
}
