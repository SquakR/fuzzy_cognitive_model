import { ModelActionType } from '../core'

export interface ConnectionConstraintInChangeType {
  hasConstraint: boolean
  minValue: number
  includeMinValue: boolean
  maxValue: number
  includeMaxValue: boolean
}

export interface ConnectionConstraintOutType {
  connectionId: number
  hasConstraint: boolean
  minValue: number
  includeMinValue: boolean
  maxValue: number
  includeMaxValue: boolean
  updatedAt: string
}

export const CHANGE_CONNECTION_CONSTRAINT_KEY = 'changeConnectionConstraint'
export type ChangeConnectionConstraintType = ModelActionType<
  typeof CHANGE_CONNECTION_CONSTRAINT_KEY,
  ConnectionConstraintOutType
>
