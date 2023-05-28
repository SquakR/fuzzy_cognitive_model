import { UseFetchOptions } from 'nuxt/app'
import {
  ChangeConceptType,
  ChangeConnectionType,
  ConceptInMoveType,
  ConceptInType,
  ConnectionInChangeType,
  ConnectionInCreateType,
  CreateConceptType,
  CreateConnectionType,
  DeleteConceptType,
  DeleteConnectionType,
  LocalFetchFuncOptions,
  LocalFetchOptions,
  ModelOutType,
  MoveConceptType,
} from '~/types'

export const useGetModel = (
  opts: LocalFetchOptions,
  projectId: number,
  fetchOptions?: UseFetchOptions<ModelOutType>
) => {
  return useLocalFetch<ModelOutType>(`/projects/${projectId}/model`, opts, {
    ...fetchOptions,
    method: 'GET',
  })
}

export const useGetModelCopy = (
  opts: LocalFetchOptions,
  modelCopyId: number,
  fetchOptions?: UseFetchOptions<ModelOutType>
) => {
  return useLocalFetch<ModelOutType>(`/models/${modelCopyId}`, opts, {
    ...fetchOptions,
    method: 'GET',
  })
}

export const useCreateConcept = (opts: LocalFetchFuncOptions) => {
  const { execute: fetch, ...rest } = useLocalFetchFunc<CreateConceptType>(
    opts,
    {
      method: 'POST',
    }
  )
  const execute = async (projectId: number, conceptIn: ConceptInType) => {
    return await fetch(`/projects/${projectId}/concept`, conceptIn)
  }
  return { execute, ...rest }
}

export const useChangeConcept = (opts: LocalFetchFuncOptions) => {
  const { execute: fetch, ...rest } = useLocalFetchFunc<ChangeConceptType>(
    opts,
    { method: 'PUT' }
  )
  const execute = async (conceptId: number, conceptIn: ConceptInType) => {
    return await fetch(`/concepts/${conceptId}`, conceptIn)
  }
  return { execute, ...rest }
}

export const useMoveConcept = (opts: LocalFetchFuncOptions) => {
  const { execute: fetch, ...rest } = useLocalFetchFunc<MoveConceptType>(opts, {
    method: 'PATCH',
  })
  const execute = async (conceptId: number, conceptIn: ConceptInMoveType) => {
    return await fetch(`/concepts/${conceptId}/move`, conceptIn)
  }
  return { execute, ...rest }
}

export const useDeleteConcept = (opts: LocalFetchFuncOptions) => {
  const { execute: fetch, ...rest } = useLocalFetchFunc<DeleteConceptType>(
    opts,
    {
      method: 'DELETE',
    }
  )
  const execute = async (conceptId: number) => {
    return await fetch(`/concepts/${conceptId}`)
  }
  return { execute, ...rest }
}

export const useCreateConnection = (opts: LocalFetchFuncOptions) => {
  const { execute: fetch, ...rest } = useLocalFetchFunc<CreateConnectionType>(
    opts,
    {
      method: 'POST',
    }
  )
  const execute = async (
    projectId: number,
    connectionIn: ConnectionInCreateType
  ) => {
    return await fetch(`/projects/${projectId}/connection`, connectionIn)
  }
  return { execute, ...rest }
}

export const useChangeConnection = (opts: LocalFetchFuncOptions) => {
  const { execute: fetch, ...rest } = useLocalFetchFunc<ChangeConnectionType>(
    opts,
    {
      method: 'PATCH',
    }
  )
  const execute = async (
    connectionId: number,
    connectionIn: ConnectionInChangeType
  ) => {
    return await fetch(`/connections/${connectionId}`, connectionIn)
  }
  return { execute, ...rest }
}

export const useDeleteConnection = (opts: LocalFetchFuncOptions) => {
  const { execute: fetch, ...rest } = useLocalFetchFunc<DeleteConnectionType>(
    opts,
    {
      method: 'DELETE',
    }
  )
  const execute = async (connectionId: number) => {
    return await fetch(`/connections/${connectionId}`)
  }
  return { execute, ...rest }
}
