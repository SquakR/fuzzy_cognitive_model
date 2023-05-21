<template>
  <template v-if="selectedConcept">
    <VTabs v-model="tab" bg-color="teal-lighten-1">
      <VTab value="concept">{{ t('concept') }}</VTab>
      <VTab v-if="plugins.adjustment.isInstalled" value="adjustment">{{
        t('adjustment')
      }}</VTab>
    </VTabs>
    <VCardText class="drawer-card-text">
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
        <VWindowItem v-if="plugins.adjustment.isInstalled" value="adjustment">
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
  </template>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { ConceptOutType, ModelOutType, Plugins } from '~/types'

export interface Props {
  model: ModelOutType
  plugins: Plugins
  selectedConcept: ConceptOutType | null
  cy: cytoscape.Core
  changeConcept: ReturnType<typeof useModelActions>['changeConcept']
  deleteConcept: ReturnType<typeof useModelActions>['deleteConcept']
  deleteConceptPending: boolean
}
defineProps<Props>()

const { t } = useI18n()

const tab = ref<'concept' | 'adjustment' | null>(null)
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
