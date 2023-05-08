import { defineStore } from 'pinia'
import { ReplaySubject, Observable, filter, map } from 'rxjs'
import { Message } from '~/types'

export const useMessageStore = defineStore('message', () => {
  const localKeys = ref<Record<string, number>>({})
  const messageStream$ = new ReplaySubject<Message>()

  const emitSuccess = (key: string, message: string) => {
    messageStream$.next({ key, type: 'success', message })
  }

  const emitError = (key: string, message: string) => {
    messageStream$.next({ key, type: 'error', message })
  }

  const subscribeGlobal = () => {
    const { success, error, subscription } = subscribe(
      messageStream$.pipe(
        filter((m) => !localKeys.value[m.key] || localKeys.value[m.key] === 0)
      )
    )
    const unsubscribe = () => {
      subscription.unsubscribe()
    }
    return { success, error, unsubscribe }
  }

  const subscribeLocal = (key: string) => {
    if (localKeys.value[key]) {
      localKeys.value[key] += 1
    } else {
      localKeys.value[key] = 1
    }
    const { success, error, subscription } = subscribe(
      messageStream$.pipe(filter((m) => m.key === key))
    )
    const unsubscribe = () => {
      localKeys.value[key] -= 1
      subscription.unsubscribe()
    }
    return {
      success,
      error,
      unsubscribe,
    }
  }

  const subscribe = (stream$: Observable<Message>) => {
    const successStream$ = pipeSuccess(stream$)
    const success = ref<string | null>(null)
    const subscription = successStream$.subscribe((m) => {
      success.value = m
      error.value = null
    })
    const errorStream$ = pipeError(stream$)
    const error = ref<string | null>(null)
    subscription.add(
      errorStream$.subscribe((m) => {
        error.value = m
        success.value = null
      })
    )
    return {
      success,
      error,
      subscription,
    }
  }

  const pipeSuccess = (stream$: Observable<Message>) => {
    return stream$.pipe(
      filter((m) => m.type === 'success'),
      map((m) => m.message)
    )
  }

  const pipeError = (stream$: Observable<Message>) => {
    return stream$.pipe(
      filter((m) => m.type === 'error'),
      map((m) => m.message)
    )
  }

  return { emitSuccess, emitError, subscribeGlobal, subscribeLocal }
})
