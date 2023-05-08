import { useLocaleStore } from '~/store'
import { UserInCreateType, UserOutType, UseInChangeType } from '~/types'

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

export const getMe = (locale: string) => {
  try {
    return localFetch<UserOutType>('/me', {
      headers: {
        'Accept-Language': locale,
      },
    })
  } catch {
    return null
  }
}

export const changeMe = async (userIn: UseInChangeType, locale: string) => {
  const formData = new FormData()
  for (const [name, value] of Object.entries(userIn)) {
    formData.append(name, value)
  }
  const response = await fetch('/user', {
    method: 'PUT',
    headers: {
      'Accept-Language': locale,
    },
    body: formData,
  })
  if (!response.ok) {
    throw createError({
      statusCode: response.status,
      statusMessage: response.statusText,
    })
  }
  return (await response.json()) as UserOutType
}
