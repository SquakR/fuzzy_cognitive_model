import { ModelActionType } from '../core'

export interface ControlConnectionOutType {
  connectionId: number
  isControl: boolean
  hasConstraint: boolean | null
  updatedAt: string
}

export const SET_IS_CONTROL_CONNECTION_KEY = 'changeControlConnection'
export type SetIsControlConnectionType = ModelActionType<
  typeof SET_IS_CONTROL_CONNECTION_KEY,
  ControlConnectionOutType
>
