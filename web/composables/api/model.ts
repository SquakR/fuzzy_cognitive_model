import {
  ConceptInCreateType,
  ConceptInMoveType,
  CreateConceptType,
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
