import { useI18n } from 'vue-i18n'
import { FetchError } from 'ofetch'
import { useTimeoutFn } from '@vueuse/core'

export interface Notification {
  type: 'success' | 'fetchError'
  message: string
}

export const useNotification = () => {
  const { t } = useI18n()

  const notificationRef = ref<Notification | null>(null)
  const intervalRef = ref<number>(Infinity)
  const { isPending, start, stop } = useTimeoutFn(() => {
    notificationRef.value = null
  }, intervalRef)

  const setNotification = (notification: Notification, interval: number) => {
    if (isPending) {
      stop()
    }
    notificationRef.value = null
    if (interval > 0) {
      notificationRef.value = notification
      intervalRef.value = interval
      if (Number.isFinite(interval)) {
        start()
      }
    }
  }

  const setSuccessNotification = (successMessage: string, interval: number) => {
    setNotification({ type: 'success', message: successMessage }, interval)
  }

  const setFetchErrorNotification = (
    fetchError: FetchError,
    interval: number
  ) => {
    setNotification(
      {
        type: 'fetchError',
        message: fetchError.message,
      },
      interval
    )
  }

  const deleteNotification = () => {
    if (isPending.value) {
      stop()
    }
    notificationRef.value = null
  }

  const message = computed(() => {
    if (!notificationRef.value) {
      return ''
    }
    if (notificationRef.value.type === 'success') {
      return t(notificationRef.value.message)
    }
    if (notificationRef.value.type === 'fetchError') {
      return notificationRef.value.message
    }
    return ''
  })

  return {
    notification: notificationRef,
    setSuccessNotification,
    setFetchErrorNotification,
    deleteNotification,
    message,
  }
}
