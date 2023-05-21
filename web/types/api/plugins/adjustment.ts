import { ModelActionType } from '../core'

export type DynamicModelType =
  | 'delta_delta'
  | 'delta_value'
  | 'value_delta'
  | 'value_value'

export interface ConceptDynamicModelOutType {
  conceptId: number
  dynamicModelType: DynamicModelType | null
  updatedAt: string
}

export interface StopConditionType {
  maxGenerations: number
  maxWithoutImprovements: number
  error: number
}

export interface AdjustmentInType {
  name: string
  description: string
  maxModelTime: string
  dynamicModelType: DynamicModelType
  generationSize: number
  generationSaveInterval: number
  stopCondition: StopConditionType
}

export interface AdjustmentRunOutType {
  id: number
  modelCopyId: number
  name: string
  description: string
  maxModelTime: number
  dynamicModelType: DynamicModelType
  generationSize: number
  generationSaveInterval: number
  stopCondition: StopConditionType
  createdAt: string
  resultChromosome: AdjustmentChromosomeGenerationOutType | null
}

export interface AdjustmentGenerationOutType {
  id: number
  number: number
  fitness: number
}

export interface AdjustmentChromosomeOutType {
  id: number
  fitness: number
  conceptValues: AdjustmentConceptValueOutType[]
  connectionValues: AdjustmentConnectionValueOutType[]
}

export interface AdjustmentChromosomeGenerationOutType {
  id: number
  fitness: number
  generationId: number
  generationNumber: number
  generationFitness: number
  conceptValues: AdjustmentConceptValueOutType[]
  connectionValues: AdjustmentConnectionValueOutType[]
}

export interface AdjustmentConceptValueOutType {
  id: number
  conceptId: number
  value: number
}

export interface AdjustmentConnectionValueOutType {
  id: number
  connectionId: number
  value: number
}

export interface AdjustmentRunsInType {
  search: string | null
  createdAtStart: string | null
  createdAtIncludeStart: boolean | null
  createdAtEnd: string | null
  createdAtIncludeEnd: boolean | null
  page: number | null
  perPage: number | null
}

export const CHANGE_DYNAMIC_MODEL_TYPE_KEY = 'changeDynamicModelType'
export type ChangeDynamicModelTypeType = ModelActionType<
  typeof CHANGE_DYNAMIC_MODEL_TYPE_KEY,
  ConceptDynamicModelOutType
>
