import {
  ChangeConceptConstraintType,
  ChangeDynamicModelTypeType,
  ChangeTargetConceptType,
  ConceptConstraintInChangeType,
  ConceptOutType,
  ConnectionOutType,
  DynamicModelType,
  ModelOutType,
  SetIsControlType,
  TargetConceptInChangeType,
} from './api'
import { LocalFetchResult } from './api-helpers'

export interface Plugins {
  getConceptClasses: (concept: ConceptOutType) => string[]
  getConnectionClasses: (connection: ConnectionOutType) => string[]
  getStyles: () => cytoscape.Stylesheet[]
  controlConcepts: ControlConceptsPlugin
  targetConcepts: TargetConceptsPlugin
  controlConnections: Plugin
  conceptConstraints: ConceptConstraintsPlugin
  connectionConstraints: Plugin
  adjustment: AdjustmentPlugin
}

export type UsePlugins = (
  model: Ref<ModelOutType>,
  cy: Ref<cytoscape.Core | null>
) => Plugins

export interface Plugin {
  isInstalled: ComputedRef<boolean>
  getConceptClasses: (concept: ConceptOutType) => string[]
  getConnectionClasses: (connection: ConnectionOutType) => string[]
  getStyles: () => cytoscape.Stylesheet[]
}

export type ControlConceptsPlugin = Plugin & {
  setIsControl: (
    conceptId: number,
    isControl: boolean
  ) => Promise<LocalFetchResult<SetIsControlType>>
  setIsControlOnSuccess: (callback: (data: SetIsControlType) => void) => void
  setIsControlPending: Ref<boolean>
  setIsControlUpdate: (result: SetIsControlType) => void
}

export type TargetConceptsPlugin = Plugin & {
  changeTargetConcept: (
    conceptId: number,
    targetConceptIn: TargetConceptInChangeType
  ) => Promise<LocalFetchResult<ChangeTargetConceptType>>
  changeTargetConceptOnSuccess: (
    callback: (data: ChangeTargetConceptType) => void
  ) => void
  changeTargetConceptPending: Ref<boolean>
  changeTargetConceptUpdate: (result: ChangeTargetConceptType) => void
}

export type ConceptConstraintsPlugin = Plugin & {
  changeConceptConstraint: (
    conceptId: number,
    conceptConstraintIn: ConceptConstraintInChangeType
  ) => Promise<LocalFetchResult<ChangeConceptConstraintType>>
  changeConceptConstraintOnSuccess: (
    callback: (data: ChangeConceptConstraintType) => void
  ) => void
  changeConceptConstraintPending: Ref<boolean>
  changeConceptConstraintUpdate: (result: ChangeConceptConstraintType) => void
}

export type AdjustmentPlugin = Plugin & {
  changeDynamicModelType: (
    conceptId: number,
    dynamicModelType: DynamicModelType | null
  ) => Promise<LocalFetchResult<ChangeDynamicModelTypeType>>
  changeDynamicModelTypeOnSuccess: (
    callback: (data: ChangeDynamicModelTypeType) => void
  ) => void
  changeDynamicModelTypePending: Ref<boolean>
  changeDynamicModelTypeUpdate: (result: ChangeDynamicModelTypeType) => void
}

export type UseControlConceptsPlugin = (
  model: Ref<ModelOutType>,
  cy: Ref<cytoscape.Core | null>
) => ControlConceptsPlugin

export type UseTargetConceptsPlugin = (
  model: Ref<ModelOutType>,
  cy: Ref<cytoscape.Core | null>
) => TargetConceptsPlugin

export type UseControlConnectionsPlugin = (
  model: Ref<ModelOutType>,
  cy: Ref<cytoscape.Core | null>
) => Plugin

export type UseConceptConstraintsPlugin = (
  model: Ref<ModelOutType>,
  cy: Ref<cytoscape.Core | null>
) => ConceptConstraintsPlugin

export type UseConnectionConstraintsPlugin = (
  model: Ref<ModelOutType>,
  cy: Ref<cytoscape.Core | null>
) => Plugin

export type UseAdjustmentPlugin = (
  model: Ref<ModelOutType>,
  cy: Ref<cytoscape.Core | null>
) => AdjustmentPlugin
