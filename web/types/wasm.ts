export type DynamicModel =
  | 'delta_delta'
  | 'delta_value'
  | 'value_delta'
  | 'value_value'

export interface Concept {
  id: number
  value: number
  isControl: boolean
  isTarget: boolean
  targetValue: TargetValue | null
  constraint: Constraint | null
  dynamicModel: DynamicModel | null
}

export interface Connection {
  id: number
  value: number
  sourceId: number
  targetId: number
  isControl: boolean
  constraint: Constraint | null
}

export interface TargetValue {
  minValue: number
  includeMinValue: boolean
  maxValue: number
  includeMaxValue: boolean
}

export interface Constraint {
  minValue: number
  includeMinValue: boolean
  maxValue: number
  includeMaxValue: boolean
}

export interface TimeSimulationData {
  time: number
  error: number
  state: Map<number, number>
}
