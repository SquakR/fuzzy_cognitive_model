<template>
  <ModelEditorToolbar v-model:mode="mode" :model="model" :plugins="plugins" />
  <ModelAddConceptForm
    v-if="cy"
    :model="model"
    :mode="mode"
    :cy="cy"
    :create-concept="createConcept"
    :create-concept-on-success="createConceptOnSuccess"
  />
  <ModelAddConnectionForm
    v-if="cy"
    ref="modelAddConnectionForm"
    :model="model"
    :mode="mode"
    :cy="cy"
    :create-connection="createConnection"
    :create-connection-on-success="createConnectionOnSuccess"
  />
  <ModelDrawer
    v-if="cy"
    :model="model"
    :plugins="plugins"
    :cy="cy"
    :change-concept="changeConcept"
    :delete-concept="deleteConcept"
    :delete-concept-pending="deleteConceptPending"
    :change-connection="changeConnection"
    :delete-connection="deleteConnection"
    :delete-connection-pending="deleteConnectionPending"
  />
  <div
    ref="container"
    class="model__cytoscape-container model-editor__cytoscape-container"
  ></div>
</template>

<script setup lang="ts">
import ModelAddConnectionForm from '~/components/model/AddConnectionForm.vue'
import { EditorMode, ModelOutType } from '~/types'

export interface Props {
  model: ModelOutType
}

const props = defineProps<Props>()

const modelAddConnectionForm = ref<InstanceType<
  typeof ModelAddConnectionForm
> | null>(null)

const { container, cy, plugins } = useModel(toRef(props, 'model'))

const mode = ref<EditorMode>('change')
watch(mode, (newValue, oldValue) => {
  if (newValue === 'change') {
    cy.value!.$('node').selectify().grabify()
    cy.value!.$('edge').selectify()
  } else {
    cy.value!.$('node').unselect().unselectify().ungrabify()
    cy.value!.$('edge').unselect().unselectify()
  }
  if (oldValue === 'addConnection') {
    modelAddConnectionForm.value!.clear()
  }
})

const {
  createConcept,
  createConceptOnSuccess,
  changeConcept,
  moveConcept,
  deleteConcept,
  deleteConceptPending,
  createConnection,
  createConnectionOnSuccess,
  changeConnection,
  deleteConnection,
  deleteConnectionPending,
} = useModelActions(toRef(props, 'model'), mode, cy, plugins)

onMounted(() => {
  cy.value!.on('select', 'node, edge', (e) =>
    cy.value!.elements().not(e.target).unselect()
  )
  cy.value!.on('drag', 'node', async (e) => {
    if (mode.value !== 'change') {
      return
    }
    cy.value!.elements().not(e.target).unselect()
    const node = e.target
    const position = node.position()
    await moveConcept(node.data().conceptId, {
      xPosition: position.x,
      yPosition: position.y,
    })
  })
})

onKeyStroke('Delete', (e) => {
  if (
    e.target instanceof HTMLInputElement ||
    e.target instanceof HTMLTextAreaElement
  ) {
    return
  }
  cy.value!.$('node:selected').forEach((node) => {
    const conceptId = node.data().conceptId
    deleteConcept(conceptId)
  })
  cy.value!.$('edge:selected').forEach((edge) => {
    const connectionId = edge.data().connectionId
    deleteConnection(connectionId)
  })
})
</script>

<style lang="sass">
.model-editor__cytoscape-container
  height: calc(100% - 48px)
</style>
