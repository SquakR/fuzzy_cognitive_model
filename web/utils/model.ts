import cytoscape from 'cytoscape'
import {
  ConceptOutType,
  ConnectionOutType,
  ModelOutType,
  Plugin,
} from '~/types'

export const getConceptId = (concept: ConceptOutType | number) => {
  const id = typeof concept === 'number' ? concept : concept.id
  return `concept${id}`
}
export const getConnectionId = (connection: ConnectionOutType) =>
  `connection${connection.id}`

export const getConceptElement = (
  model: ModelOutType,
  concept: ConceptOutType,
  locale: string,
  plugins: Plugin
): cytoscape.ElementDefinition => {
  return {
    data: {
      conceptId: concept.id,
      id: getConceptId(concept),
      label:
        model.project.conceptValueType === 'none'
          ? ''
          : new Intl.NumberFormat(locale).format(concept.value!),
    },
    classes: plugins.getConceptClasses(concept).join(' '),
  }
}

export const setConceptPosition = (
  cy: cytoscape.Core,
  concept: ConceptOutType
) => {
  cy?.$(`#${getConceptId(concept.id)}`).position({
    x: concept.xPosition,
    y: concept.yPosition,
  })
}
