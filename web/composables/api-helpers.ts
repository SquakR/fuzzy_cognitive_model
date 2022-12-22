import { FetchRequest, FetchOptions } from 'ofetch'

export const useLocalFetch: typeof useFetch = (request, opts) => {
  const config = useRuntimeConfig()
  return useFetch(request, {
    baseURL: config.public.API_HTTP_BASE_URL,
    ...opts,
  })
}

export const useLocalLazyFetch: typeof useLazyFetch = (request, opts) => {
  const config = useRuntimeConfig()
  return useLazyFetch(request, {
    baseURL: config.public.API_HTTP_BASE_URL,
    ...opts,
  })
}

export const localFetch = <T>(request: FetchRequest, opts?: FetchOptions) => {
  const config = useRuntimeConfig()
  return $fetch<T>(request, {
    baseURL: config.public.API_HTTP_BASE_URL,
    ...opts,
  })
}
