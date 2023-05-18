import {
  ConceptOutDeleteType,
  ConceptOutMoveType,
  ConceptOutType,
  ConnectionOutDeleteType,
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

export const CHANGE_CONCEPT_KEY = 'changeConcept'
export type ChangeConceptType = ModelActionType<
  typeof CHANGE_CONCEPT_KEY,
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

export const DELETE_CONNECTION_KEY = 'deleteConnection'
export type DeleteConnectionType = ModelActionType<
  typeof DELETE_CONNECTION_KEY,
  ConnectionOutDeleteType
>

export type ModelActionResult =
  | ModelActionErrorType
  | CreateConceptType
  | ChangeConceptType
  | MoveConceptType
  | DeleteConceptType
  | CreateConnectionType
  | DeleteConnectionType
