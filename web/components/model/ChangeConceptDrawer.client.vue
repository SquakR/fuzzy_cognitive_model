<template>
  <Teleport to="#change-concept-drawer">
    <VExpandXTransition>
      <VCard
        v-show="isActive"
        class="drawer-card"
        :width="tab === 'adjustment' ? 520 : 500"
      >
        <VTabs v-model="tab" bg-color="teal-lighten-1">
          <VTab value="concept">{{ t('concept') }}</VTab>
          <VTab v-if="plugins.adjustment.isInstalled" value="adjustment">{{
            t('adjustment')
          }}</VTab>
        </VTabs>
        <VCardText v-if="selectedConcept" class="drawer-card-text">
          <VWindow v-model="tab">
            <VWindowItem value="concept"
              ><ModelChangeConceptForm
                :model="model"
                :cy="cy"
                :selected-concept="selectedConcept"
                :concept-constraints-plugin="plugins.conceptConstraints"
                :change-concept="changeConcept"
                :delete-concept="deleteConcept"
                :delete-concept-pending="deleteConceptPending"
            /></VWindowItem>
            <VWindowItem
              v-if="plugins.adjustment.isInstalled"
              value="adjustment"
            >
              <PluginsControlConceptsChangeConceptForm
                :selected-concept="selectedConcept"
                :control-concepts-plugin="plugins.controlConcepts"
              />
              <VDivider />
              <PluginsConceptConstraintsChangeConceptForm
                :selected-concept="selectedConcept"
                :concept-constraints-plugin="plugins.conceptConstraints"
              />
              <VDivider />
              <PluginsTargetConceptsChangeConceptForm
                :selected-concept="selectedConcept"
                :target-concepts-plugin="plugins.targetConcepts"
              />
              <VDivider />
              <PluginsAdjustmentChangeConceptForm
                :selected-concept="selectedConcept"
                :adjustment-plugin="plugins.adjustment"
              />
              <VDivider />
            </VWindowItem>
          </VWindow>
        </VCardText>
      </VCard>
    </VExpandXTransition>
  </Teleport>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { ConceptOutType, ModelOutType, Plugins } from '~/types'

export interface Props {
  model: ModelOutType
  plugins: Plugins
  cy: cytoscape.Core
  changeConcept: ReturnType<typeof useModelActions>['changeConcept']
  deleteConcept: ReturnType<typeof useModelActions>['deleteConcept']
  deleteConceptPending: boolean
}
const props = defineProps<Props>()

const { t } = useI18n()

const selectedConcept = ref<ConceptOutType | null>(null)
const isActive = computed(() => Boolean(selectedConcept.value))

const tab = ref<'concept' | 'adjustment' | null>(null)

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
</script>

<i18n locale="en-US" lang="json">
{
  "concept": "Concept",
  "adjustment": "Adjustment"
}
</i18n>

<i18n locale="ru-RU" lang="json">
{
  "concept": "Концепт",
  "adjustment": "Настройка"
}
</i18n>
