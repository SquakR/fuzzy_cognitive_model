import {
  ConceptOutDeleteType,
  ConceptOutMoveType,
  ConceptOutType,
  ConnectionOutType,
} from './core'

export interface ModelActionType<N, T> {
  projectId: number
  projectUpdatedAt: string
  name: N
  data: T
}

export interface ModelActionErrorType {
  projectId: number
  name: string
  message: string
}

export const CREATE_CONCEPT_KEY = 'createConcept'
export type CreateConceptType = ModelActionType<
  typeof CREATE_CONCEPT_KEY,
  ConceptOutType
>

export const MOVE_CONCEPT_KEY = 'moveConcept'
export type MoveConceptType = ModelActionType<
  typeof MOVE_CONCEPT_KEY,
  ConceptOutMoveType
>

export const DELETE_CONCEPT_KEY = 'deleteConcept'
export type DeleteConceptType = ModelActionType<
  typeof DELETE_CONCEPT_KEY,
  ConceptOutDeleteType
>

export const CREATE_CONNECTION_KEY = 'createConnection'
export type CreateConnectionType = ModelActionType<
  typeof CREATE_CONNECTION_KEY,
  ConnectionOutType
>

export type ModelActionResult =
  | ModelActionErrorType
  | CreateConceptType
  | MoveConceptType
  | DeleteConceptType
  | CreateConnectionType
