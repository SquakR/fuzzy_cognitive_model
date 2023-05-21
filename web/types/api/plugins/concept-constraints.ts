import { ModelActionType } from '../core'

export interface ConceptConstraintInChangeType {
  hasConstraint: boolean
  minValue: number
  includeMinValue: boolean
  maxValue: number
  includeMaxValue: boolean
}

export interface ConceptConstraintOutType {
  conceptId: number
  hasConstraint: boolean
  minValue: number
  includeMinValue: boolean
  maxValue: number
  includeMaxValue: boolean
  updatedAt: string
}

export const CHANGE_CONCEPT_CONSTRAINT_KEY = 'changeConceptConstraint'
export type ChangeConceptConstraintType = ModelActionType<
  typeof CHANGE_CONCEPT_CONSTRAINT_KEY,
  ConceptConstraintOutType
>
