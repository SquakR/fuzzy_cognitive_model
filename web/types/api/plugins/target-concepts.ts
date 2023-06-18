import { ModelActionType } from '../core'

export interface TargetConceptInChangeType {
  isTarget: boolean
  minValue: number
  includeMinValue: boolean
  maxValue: number
  includeMaxValue: boolean
}

export interface TargetConceptOutType {
  conceptId: number
  isTarget: boolean
  minValue: number
  includeMinValue: boolean
  maxValue: number
  includeMaxValue: boolean
  updatedAt: string
}

export const CHANGE_TARGET_CONCEPT_KEY = 'changeTargetConcept'
export type ChangeTargetConceptType = ModelActionType<
  typeof CHANGE_TARGET_CONCEPT_KEY,
  TargetConceptOutType
>
