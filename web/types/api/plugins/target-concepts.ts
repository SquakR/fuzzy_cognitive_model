import { ModelActionType } from '../core'

export interface TargetConceptInChangeType {
  isTarget: boolean
  value: number | null
}

export interface TargetConceptOutType {
  conceptId: number
  isTarget: boolean
  value: number | null
  updatedAt: string
}

export const CHANGE_TARGET_CONCEPT_KEY = 'changeTargetConcept'
export type ChangeTargetConceptType = ModelActionType<
  typeof CHANGE_TARGET_CONCEPT_KEY,
  TargetConceptOutType
>
