import {
  ConceptInMoveType,
  ConceptOutType,
  LocalFetchFuncOptions,
  LocalFetchOptions,
  ModelActionType,
  ModelOutType,
} from '~/types'

export const useGetModel = (opts: LocalFetchOptions, projectId: number) => {
  return useLocalFetch<ModelOutType>(`/projects/${projectId}`, opts)
}

export const useMoveConcept = (opts: LocalFetchFuncOptions) => {
  const { execute: fetch, ...rest } = useLocalFetchFunc<
    ModelActionType<ConceptOutType>
  >(opts, {
    method: 'PATCH',
  })
  const execute = async (conceptId: number, conceptIn: ConceptInMoveType) => {
    return await fetch(`/concepts/${conceptId}/move`, conceptIn)
  }
  return { execute, ...rest }
}
