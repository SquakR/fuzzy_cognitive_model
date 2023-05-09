import {
  UserInCreateType,
  UserOutType,
  CredentialsType,
  SessionType,
  LocalFetchOptions,
  LocalFetchFuncOptions,
  ProjectsInType,
  PaginationOutType,
  ProjectOutType,
  ProjectInType,
} from '~/types'

export const useCreateUser = (opts: LocalFetchFuncOptions<UserOutType>) => {
  const fetch = useLocalFetchFormDataFunc('/user', opts, {
    method: 'POST',
  })
  return async (userIn: UserInCreateType) => {
    return await fetch(userIn)
  }
}

export const useGetMe = (opts: LocalFetchFuncOptions<UserOutType>) => {
  return useLocalFetchFunc('/me', opts)
}

export const useChangeMeLocale = (opts: LocalFetchFuncOptions<UserOutType>) => {
  const fetch = useLocalFetchFunc('me/locale', opts, {
    method: 'PATCH',
  })
  return (newLocale: string) => {
    return fetch(JSON.stringify(newLocale))
  }
}

export const useSignIn = (opts: LocalFetchFuncOptions<SessionType>) => {
  const fetch = useLocalFetchFunc('/sign_in', opts, {
    method: 'POST',
  })
  return (credentials: CredentialsType) => {
    return fetch(credentials)
  }
}

export const useSignOut = (opts: LocalFetchFuncOptions<null>) => {
  const fetch = useLocalFetchFunc('/sign_out', opts, {
    method: 'PATCH',
  })
  return () => {
    return fetch()
  }
}

export const useGetProjects = (
  opts: LocalFetchOptions,
  projectsIn: Ref<ProjectsInType>
) => {
  return useLocalFetch<PaginationOutType<ProjectOutType>>('/projects', opts, {
    params: computed(() =>
      Object.fromEntries(
        Object.entries(projectsIn.value).filter(([_, v]) => !!v)
      )
    ),
  })
}

export const useCreateProject = (
  opts: LocalFetchFuncOptions<ProjectOutType>
) => {
  const fetch = useLocalFetchFunc('/project', opts, {
    method: 'POST',
  })
  return async (projectIn: ProjectInType) => {
    return await fetch(projectIn)
  }
}
