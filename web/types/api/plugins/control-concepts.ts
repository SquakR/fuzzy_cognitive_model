import { ModelActionType } from '../core'

export interface ControlConceptOutType {
  conceptId: number
  isControl: boolean
  hasConstraint: boolean | null
  updatedAt: string
}

export const SET_IS_CONTROL_KEY = 'changeControlConcept'
export type SetIsControlType = ModelActionType<
  typeof SET_IS_CONTROL_KEY,
  ControlConceptOutType
>
