import { useLocaleStore } from '~/store'
import {
  UserInCreateType,
  UserOutType,
  CredentialsType,
  SessionType,
} from '~/types'

export const useCreateUser = () => {
  const config = useRuntimeConfig()
  const storeLocale = useLocaleStore()
  return async (userIn: UserInCreateType) => {
    const formData = new FormData()
    for (const [name, value] of Object.entries(userIn)) {
      formData.append(name, value)
    }
    const response = await fetch(`${config.public.API_HTTP_BASE_URL}/user`, {
      method: 'POST',
      headers: {
        'Accept-Language': userIn.language || storeLocale.locale,
      },
      body: formData,
    })
    if (!response.ok) {
      throw createError({
        statusCode: response.status,
        statusMessage: response.statusText,
        message: await response.text(),
      })
    }
    return (await response.json()) as UserOutType
  }
}

export const useSignIn = () => {
  const fetch = useLocalFetchFunc()
  return async (credentials: CredentialsType) => {
    return fetch<SessionType>('/sign_in', {
      method: 'POST',
      body: credentials,
    })
  }
}
