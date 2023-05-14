import { ConceptOutMoveType } from './core'

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

export const MOVE_CONCEPT_KEY = 'moveConcept'
export type MoveConceptType = ModelActionType<
  typeof MOVE_CONCEPT_KEY,
  ConceptOutMoveType
>

export type ModelActionResult = ModelActionErrorType | MoveConceptType
