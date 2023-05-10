import {
  UserInCreateType,
  UserOutType,
  CredentialsType,
  SessionType,
  LocalFetchFuncOptions,
} from '~/types'

export const useCreateUser = (opts: LocalFetchFuncOptions) => {
  const { execute: fetch, ...rest } = useLocalFetchFormDataFunc<UserOutType>(
    opts,
    {
      method: 'POST',
    }
  )
  const execute = async (userIn: UserInCreateType) => {
    return await fetch('/user', userIn)
  }
  return { execute, ...rest }
}

export const useGetMe = (opts: LocalFetchFuncOptions) => {
  const { execute: fetch, ...rest } = useLocalFetchFunc<UserOutType>(opts)
  const execute = () => fetch('/me')
  return { execute, ...rest }
}

export const useChangeMeLocale = (opts: LocalFetchFuncOptions) => {
  const { execute: fetch, ...rest } = useLocalFetchFunc<UserOutType>(opts, {
    method: 'PATCH',
  })
  const execute = (newLocale: string) => {
    return fetch('me/locale', JSON.stringify(newLocale))
  }
  return { execute, ...rest }
}

export const useSignIn = (opts: LocalFetchFuncOptions) => {
  const { execute: fetch, ...rest } = useLocalFetchFunc<SessionType>(opts, {
    method: 'POST',
  })
  const execute = (credentials: CredentialsType) => {
    return fetch('/sign_in', credentials)
  }
  return { execute, ...rest }
}

export const useSignOut = (opts: LocalFetchFuncOptions) => {
  const { execute: fetch, ...rest } = useLocalFetchFunc<null>(opts, {
    method: 'PATCH',
  })
  const execute = () => {
    return fetch('/sign_out')
  }
  return { execute, ...rest }
}
