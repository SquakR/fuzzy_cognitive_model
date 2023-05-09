import { UseFetchOptions, FetchResult as NuxtFetchResult } from 'nuxt/app'
import { KeysOf } from 'nuxt/dist/app/composables/asyncData'
import type { NitroFetchOptions, NitroFetchRequest } from 'nitropack'
import { FetchRequest, FetchError } from 'ofetch'
import { useMessageStore, useUserStore } from '~/store'
import { LocalFetchFuncOptions, LocalFetchOptions } from '~/types'

export const useLocalFetch = <
  ResT = void,
  _ResT = ResT extends void ? NuxtFetchResult<NitroFetchRequest, 'get'> : ResT,
  DataT = _ResT,
  PickKeys extends KeysOf<DataT> = KeysOf<DataT>
>(
  request:
    | Ref<NitroFetchRequest>
    | NitroFetchRequest
    | (() => NitroFetchRequest),
  opts: LocalFetchOptions,
  fetchOpts?: UseFetchOptions<_ResT, DataT, PickKeys, NitroFetchRequest, 'get'>
) => {
  const config = useRuntimeConfig()
  const userStore = useUserStore()
  const headers = useRequestHeaders(['cookie'])
  const messageStore = useMessageStore()

  const { error, ...rest } = useFetch<
    ResT,
    FetchError,
    NitroFetchRequest,
    'get',
    _ResT,
    DataT,
    PickKeys
  >(request, {
    key: opts.key,
    baseURL: config.public.API_HTTP_BASE_URL,
    headers: {
      'Accept-Language': userStore.locale,
      ...headers,
    },
    credentials: 'include',
    ...fetchOpts,
  })
  watch(error, (newValue) => {
    if (newValue && (opts.emitError === undefined || opts.emitError)) {
      messageStore.emitError(opts.key, String(newValue.data))
    }
  })
  return { error, ...rest }
}

export const useLocalFetchFormDataFunc = <T>(
  url: string,
  opts: LocalFetchFuncOptions<T>,
  fetchOpts?: RequestInit
) => {
  const config = useRuntimeConfig()
  const userStore = useUserStore()
  const headers = useRequestHeaders(['cookie'])
  const messageStore = useMessageStore()

  return async (values: Record<string, any>) => {
    const formData = new FormData()
    for (const [name, value] of Object.entries(values)) {
      formData.append(name, value)
    }
    const response = await fetch(`${config.public.API_HTTP_BASE_URL}${url}`, {
      headers: {
        'Accept-Language': userStore.locale,
        ...headers,
      },
      credentials: 'include',
      body: formData,
      ...fetchOpts,
    })
    if (!response.ok) {
      const errorMessage = await response.text()
      if (opts.onError) {
        opts.onError(errorMessage)
      }
      if (opts.emitError === undefined || opts.emitError) {
        messageStore.emitError(opts.key, errorMessage)
      }
      return null
    } else {
      const result = (await response.json()) as T
      if (opts.onSuccess) {
        opts.onSuccess(result)
      }
      if (opts.successMessage) {
        messageStore.emitSuccess(opts.key, opts.successMessage)
      }
      return result
    }
  }
}

export const useLocalFetchFunc = <T>(
  request: FetchRequest,
  opts: LocalFetchFuncOptions<T>,
  fetchOpts?: NitroFetchOptions<NitroFetchRequest>
) => {
  const config = useRuntimeConfig()
  const userStore = useUserStore()
  const headers = useRequestHeaders(['cookie'])
  const messageStore = useMessageStore()

  return async (body?: RequestInit['body'] | Record<string, any>) => {
    try {
      const result = await $fetch<T>(request, {
        baseURL: config.public.API_HTTP_BASE_URL,
        headers: {
          'Accept-Language': userStore.locale,
          ...headers,
        },
        credentials: 'include',
        body,
        ...fetchOpts,
      })
      if (opts.onSuccess) {
        opts.onSuccess(result)
      }
      if (opts.successMessage) {
        messageStore.emitSuccess(opts.key, opts.successMessage)
      }
      return result
    } catch (error) {
      if (error instanceof FetchError) {
        const errorMessage = String(error.data)
        if (opts.onError) {
          opts.onError(errorMessage)
        }
        if (opts.emitError === undefined || opts.emitError) {
          messageStore.emitError(opts.key, errorMessage)
        }
        return null
      }
      throw error
    }
  }
}
