import {
  ConceptInCreateType,
  ConceptInMoveType,
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

export const useGetModel = (opts: LocalFetchOptions, projectId: number) => {
  return useLocalFetch<ModelOutType>(`/projects/${projectId}`, opts)
}

export const useCreateConcept = (opts: LocalFetchFuncOptions) => {
  const { execute: fetch, ...rest } = useLocalFetchFunc<CreateConceptType>(
    opts,
    {
      method: 'POST',
    }
  )
  const execute = async (projectId: number, conceptIn: ConceptInCreateType) => {
    return await fetch(`/projects/${projectId}/concept`, conceptIn)
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
