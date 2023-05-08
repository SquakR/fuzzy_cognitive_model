import { FetchRequest, FetchOptions } from 'ofetch'
import { useLocaleStore } from '~/store'

export const useLocalFetchFunc = () => {
  const config = useRuntimeConfig()
  const storeLocale = useLocaleStore()
  return <T>(request: FetchRequest, opts?: FetchOptions) => {
    // @ts-ignore
    return $fetch<T>(request, {
      baseURL: config.public.API_HTTP_BASE_URL,
      headers: {
        'Accept-Language': storeLocale.locale,
      },
      credentials: 'include',
      ...opts,
    })
  }
}
