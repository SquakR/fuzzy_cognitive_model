import { FetchError } from 'ofetch'

export type Message = {
  id: number
  type: 'success' | 'fetchError'
  message: string
}

export const useMessages = () => {
  const nextId = ref<number>(0)
  const messages = ref<Message[]>([])

  const getNextId = () => {
    const id = nextId.value
    nextId.value++
    return id
  }

  const addMessage = (message: Message, interval: number) => {
    if (interval > 0) {
      messages.value.push(message)
      const id = message.id
      if (Number.isFinite(interval)) {
        setTimeout(() => deleteMessage(id), interval)
      }
    }
  }

  const addSuccessMessage = (successMessage: string, interval: number) => {
    addMessage(
      { id: getNextId(), type: 'success', message: successMessage },
      interval
    )
  }

  const addFetchErrorMessage = (fetchError: FetchError, interval: number) => {
    addMessage(
      {
        id: getNextId(),
        type: 'fetchError',
        message: fetchError.message,
      },
      interval
    )
  }

  const deleteMessage = (id: number) => {
    messages.value = messages.value.filter((message) => message.id !== id)
  }

  return {
    messages,
    addSuccessMessage,
    addFetchErrorMessage,
    deleteMessage,
  }
}
