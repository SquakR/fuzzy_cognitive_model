<template>
  <Teleport to=".editor-layout__drawer">
    <VExpandXTransition>
      <VCard v-show="isActive" ref="drawerCard" class="drawer-card">
        <ModelChangeConceptDrawerContent
          :model="model"
          :plugins="plugins"
          :selected-concept="selectedConcept"
          :cy="cy"
          :change-concept="changeConcept"
          :delete-concept="deleteConcept"
          :delete-concept-pending="deleteConceptPending"
        />
        <ModelChangeConnectionDrawerContent
          :model="model"
          :plugins="plugins"
          :selected-connection="selectedConnection"
          :cy="cy"
          :change-connection="changeConnection"
          :delete-connection="deleteConnection"
          :delete-connection-pending="deleteConnectionPending"
        />
      </VCard>
    </VExpandXTransition>
  </Teleport>
</template>

<script setup lang="ts">
import { VCard } from 'vuetify/components/VCard'
import {
  ConceptOutType,
  ConnectionOutType,
  ModelOutType,
  Plugins,
} from '~/types'

export interface Props {
  model: ModelOutType
  plugins: Plugins
  cy: cytoscape.Core
  changeConcept: ReturnType<typeof useModelActions>['changeConcept']
  deleteConcept: ReturnType<typeof useModelActions>['deleteConcept']
  deleteConceptPending: boolean
  changeConnection: ReturnType<typeof useModelActions>['changeConnection']
  deleteConnection: ReturnType<typeof useModelActions>['deleteConnection']
  deleteConnectionPending: boolean
}
const props = defineProps<Props>()

const selectedConcept = ref<ConceptOutType | null>(null)
const selectedConnection = ref<ConnectionOutType | null>(null)

const isActive = computed(
  () => Boolean(selectedConcept.value) || Boolean(selectedConnection.value)
)

props.cy.on('select', 'node', (e) => {
  selectedConcept.value = props.model.concepts.find(
    (concept) => concept.id === e.target.data().conceptId
  )!
})
props.cy.on('unselect', 'node', () => {
  selectedConcept.value = null
})
props.cy.on('remove', 'node', () => {
  selectedConcept.value = null
})

props.cy.on('select', 'edge', (e) => {
  selectedConnection.value = props.model.connections.find(
    (connection) => connection.id === e.target.data().connectionId
  )!
})
props.cy.on('unselect', 'edge', () => {
  selectedConnection.value = null
})
props.cy.on('remove', 'edge', () => {
  selectedConnection.value = null
})
</script>
