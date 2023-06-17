import { defu } from 'defu'
import type { NitroFetchOptions, NitroFetchRequest } from 'nitropack'
import { UseFetchOptions } from 'nuxt/app'
import { FetchError, FetchRequest } from 'ofetch'
import { useMessageStore, useUserStore } from '~/store'
import {
  ErrorPayload,
  LocalFetchFuncOptions,
  LocalFetchOptions,
  LocalFetchResult,
} from '~/types'

export const useLocalFetchRaw = <T>(
  request:
    | Ref<NitroFetchRequest>
    | NitroFetchRequest
    | (() => NitroFetchRequest),
  fetchOpts?: UseFetchOptions<T>
) => {
  const config = useRuntimeConfig()
  const userStore = useUserStore()
  const headers = useRequestHeaders(['cookie'])

  const defaults: UseFetchOptions<T> = {
    baseURL: config.public.API_HTTP_BASE_URL,
    headers: {
      'Accept-Language': userStore.locale,
      ...headers,
    },
    credentials: 'include',
  }

  return useFetch(request, defu(fetchOpts, defaults))
}

export const useLocalFetch = async <T>(
  request:
    | Ref<NitroFetchRequest>
    | NitroFetchRequest
    | (() => NitroFetchRequest),
  opts: LocalFetchOptions,
  fetchOpts?: UseFetchOptions<T>
) => {
  const messageStore = useMessageStore()

  const defaults: UseFetchOptions<T> = {
    key: opts.key,
  }

  const { error, ...rest } = await useLocalFetchRaw(
    request,
    defu(fetchOpts, defaults)
  )
  watch(
    error,
    (newValue) => {
      if (newValue) {
        if (opts.fatal === undefined || opts.fatal) {
          throw createError({
            statusCode: newValue.statusCode,
            statusMessage: newValue.statusMessage,
            data: newValue.data,
            fatal: true,
          })
        }
        if (opts.emitError === undefined || opts.emitError) {
          messageStore.emitError(opts.key, getErrorMessage(newValue.data))
        }
      }
    },
    { immediate: true }
  )
  return { error, ...rest }
}

export const useLocalFetchFuncRaw = <T>(
  fetchOpts?: NitroFetchOptions<NitroFetchRequest>
) => {
  const config = useRuntimeConfig()
  const userStore = useUserStore()
  const headers = useRequestHeaders(['cookie'])

  const onSuccessHandlers = ref<((data: T) => void)[]>([])
  const onErrorHandlers = ref<((errorData: string | ErrorPayload) => void)[]>(
    []
  )

  const onSuccess = (callback: (data: T) => void) => {
    onSuccessHandlers.value.push(callback)
  }
  const onError = (callback: (errorData: string | ErrorPayload) => void) => {
    onErrorHandlers.value.push(callback)
  }

  const pending = ref(false)

  const execute = async (
    request: FetchRequest,
    body?: RequestInit['body'] | Record<string, any>
  ): Promise<LocalFetchResult<T>> => {
    pending.value = true
    try {
      const data = await $fetch<T>(request, {
        baseURL: config.public.API_HTTP_BASE_URL,
        headers: {
          'Accept-Language': userStore.locale,
          ...headers,
        },
        credentials: 'include',
        body,
        ...fetchOpts,
      })
      for (const onSuccess of onSuccessHandlers.value) {
        onSuccess(data)
      }
      return { data, success: true, errorData: null }
    } catch (error) {
      if (error instanceof FetchError) {
        for (const onError of onErrorHandlers.value) {
          onError(error.data)
        }
        return { data: null, success: false, errorData: error.data }
      }
      throw error
    } finally {
      pending.value = false
    }
  }

  return { execute, pending, onSuccess, onError }
}

export const useLocalFetchFunc = <T>(
  opts: LocalFetchFuncOptions,
  fetchOpts?: NitroFetchOptions<NitroFetchRequest>
) => {
  const messageStore = useMessageStore()
  const { onSuccess, onError, ...rest } = useLocalFetchFuncRaw<T>(fetchOpts)
  onSuccess(() => {
    if (opts.successMessage) {
      messageStore.emitSuccess(opts.key, opts.successMessage)
    } else {
      messageStore.emitClear(opts.key)
    }
  })
  onError((errorData: string | ErrorPayload) => {
    const errorMessage = getErrorMessage(errorData)
    if (opts.emitError === undefined || opts.emitError) {
      messageStore.emitError(opts.key, errorMessage)
    } else {
      messageStore.emitClear(opts.key)
    }
  })
  return { onSuccess, onError, ...rest }
}

export const useLocalFetchFormDataFunc = <T>(
  opts: LocalFetchFuncOptions,
  fetchOpts?: NitroFetchOptions<NitroFetchRequest>
) => {
  const { execute: fetch, ...rest } = useLocalFetchFunc<T>(opts, fetchOpts)
  const execute = async (
    request: FetchRequest,
    values: Record<string, any>
  ) => {
    const formData = new FormData()
    for (const [name, value] of Object.entries(values)) {
      formData.append(name, value)
    }
    return fetch(request, formData)
  }
  return { execute, ...rest }
}

const getErrorMessage = (errorData: string | ErrorPayload) => {
  if (typeof errorData === 'string') {
    return errorData
  }
  return `${errorData.error.reason}. ${errorData.error.description}`
}
