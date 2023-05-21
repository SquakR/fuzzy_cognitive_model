import cytoscape from 'cytoscape'
import {
  ConceptOutType,
  ConnectionOutType,
  ModelOutType,
  Plugins,
} from '~/types'

export const getConceptId = (concept: ConceptOutType | number) => {
  const id = typeof concept === 'number' ? concept : concept.id
  return `concept${id}`
}
export const getConnectionId = (connection: ConnectionOutType | number) => {
  const id = typeof connection === 'number' ? connection : connection.id
  return `connection${id}`
}

export const createConceptElement = (
  model: ModelOutType,
  concept: ConceptOutType,
  locale: string,
  plugins: Plugins
): cytoscape.ElementDefinition => ({
  data: createConceptData(model, concept, locale),
  classes: plugins.getConceptClasses(concept).join(' '),
})

export const createConceptData = (
  model: ModelOutType,
  concept: ConceptOutType,
  locale: string
) => {
  return {
    conceptId: concept.id,
    id: getConceptId(concept),
    label:
      model.project.conceptValueType === 'none'
        ? ''
        : new Intl.NumberFormat(locale).format(concept.value!),
  }
}

export const createConnectionElement = (
  connection: ConnectionOutType,
  locale: string,
  plugins: Plugins
): cytoscape.ElementDefinition => ({
  data: createConnectionData(connection, locale),
  classes: plugins.getConnectionClasses(connection).join(' '),
})

export const createConnectionData = (
  connection: ConnectionOutType,
  locale: string
) => {
  return {
    connectionId: connection.id,
    id: getConnectionId(connection),
    source: getConceptId(connection.sourceId),
    target: getConceptId(connection.targetId),
    label: new Intl.NumberFormat(locale).format(connection.value),
  }
}

export const setConceptDataWithPosition = (
  cy: cytoscape.Core,
  model: ModelOutType,
  concept: ConceptOutType,
  locale: string
) => {
  cy.$(`#${getConceptId(concept.id)}`)
    .data(createConceptData(model, concept, locale))
    .position({
      x: concept.xPosition,
      y: concept.yPosition,
    })
}

export const setConceptPosition = (
  cy: cytoscape.Core,
  concept: ConceptOutType
) => {
  cy.$(`#${getConceptId(concept.id)}`).position({
    x: concept.xPosition,
    y: concept.yPosition,
  })
}

export const setConnectionData = (
  cy: cytoscape.Core,
  connection: ConnectionOutType,
  locale: string
) => {
  cy.$(`#${getConnectionId(connection.id)}`).data(
    createConnectionData(connection, locale)
  )
}
