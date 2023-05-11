import {
  LocalFetchFuncOptions,
  LocalFetchOptions,
  PaginationOutType,
  PluginType,
  ProjectInType,
  ProjectOutType,
  ProjectsInType,
} from '~/types'

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

export const useCreateProject = (opts: LocalFetchFuncOptions) => {
  const { execute: fetch, ...rest } = useLocalFetchFunc<ProjectOutType>(opts, {
    method: 'POST',
  })
  const execute = async (projectIn: ProjectInType) => {
    return await fetch('/project', projectIn)
  }
  return { execute, ...rest }
}

export const useGetPlugins = (opts: LocalFetchOptions) => {
  return useLocalFetch<PluginType[]>('/plugins', opts)
}

export const useChangeProject = (opts: LocalFetchFuncOptions) => {
  const { execute: fetch, ...rest } = useLocalFetchFunc<ProjectOutType>(opts, {
    method: 'PUT',
  })
  const execute = async (projectId: number, projectIn: ProjectInType) => {
    return await fetch(`/projects/${projectId}`, projectIn)
  }
  return { execute, ...rest }
}

export const useSetProjectPlugins = (opts: LocalFetchFuncOptions) => {
  const { execute: fetch, ...rest } = useLocalFetchFunc<string[]>(opts, {
    method: 'POST',
  })
  const execute = async (projectId: number, newPlugins: string[]) => {
    return await fetch(`/projects/${projectId}/plugins`, newPlugins)
  }
  return { execute, ...rest }
}

export const useDeleteProject = (opts: LocalFetchFuncOptions) => {
  const { execute: fetch, ...rest } = useLocalFetchFunc<null>(opts, {
    method: 'DELETE',
  })
  const execute = async (projectId: number) => {
    return await fetch(`/projects/${projectId}`)
  }
  return { execute, ...rest }
}
