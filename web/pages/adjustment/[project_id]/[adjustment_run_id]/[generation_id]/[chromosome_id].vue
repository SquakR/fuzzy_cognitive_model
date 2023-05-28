<template>
  <PluginsAdjustmentBreadcrumbs :items="bc" />
  <VCard height="calc(100% - 40px)">
    <VCardTitle class="d-flex">
      {{ t('title', { fitness }) }}
      <VSpacer />
      <PluginsAdjustmentModelButton
        :project-id="Number($route.params.project_id)"
      />
    </VCardTitle>
    <VCardText class="chromosome-id__card-text">
      <PluginsAdjustmentEditor :model="model" />
    </VCardText>
  </VCard>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { useUserStore } from '~/store'
import { BreadcrumbsItem } from '~/types'

definePageMeta({
  layout: 'model',
  middleware: 'auth',
})

const { t } = useI18n()
const route = useRoute()
const userStore = useUserStore()

const [
  { data: project },
  { data: adjustmentRun },
  { data: adjustmentGeneration },
  { data: adjustmentChromosome },
] = await Promise.all([
  useGetProject({ key: 'project' }, Number(route.params.project_id), {
    pick: ['name'],
  }),
  useGetAdjustmentRun(
    { key: 'adjustmentRun' },
    Number(route.params.adjustment_run_id),
    { pick: ['name', 'modelCopyId'] }
  ),
  useGetAdjustmentGeneration(
    {
      key: 'adjustmentGeneration',
    },
    Number(route.params.generation_id),
    { pick: ['number'] }
  ),
  useGetAdjustmentChromosome(
    {
      key: 'adjustmentChromosome',
    },
    Number(route.params.chromosome_id)
  ),
])

const { data: modelData } = await useGetModelCopy(
  { key: 'modelCopy' },
  adjustmentRun.value!.modelCopyId
)
const model = computed(() => ({
  project: { ...modelData.value!.project },
  concepts: modelData.value!.concepts.map((concept) => {
    const newConcept = { ...concept }
    const conceptValue = adjustmentChromosome.value!.conceptValues.find(
      (conceptValue) => conceptValue.conceptId === concept.id
    )
    if (conceptValue) {
      newConcept.value = conceptValue.value
    }
    return newConcept
  }),
  connections: modelData.value!.connections.map((connection) => {
    const newConnection = { ...connection }
    const connectionValue = adjustmentChromosome.value!.connectionValues.find(
      (connectionValue) => connectionValue.connectionId === connectionValue.id
    )
    if (connectionValue) {
      newConnection.value = connectionValue.value
    }
    return newConnection
  }),
}))

const bc = computed<BreadcrumbsItem[]>(() => [
  {
    title: project.value!.name,
    to: {
      name: 'adjustment-project_id',
      params: { project_id: route.params.project_id },
    },
  },
  {
    title: adjustmentRun.value!.name,
    to: {
      name: 'adjustment-project_id-adjustment_run_id',
      params: {
        project_id: route.params.project_id,
        adjustment_run_id: route.params.adjustment_run_id,
      },
    },
  },
  {
    title: String(adjustmentGeneration.value!.number),
    to: {
      name: 'adjustment-project_id-adjustment_run_id-generation_id',
      params: {
        project_id: route.params.project_id,
        adjustment_run_id: route.params.adjustment_run_id,
        generation_id: route.params.generation_id,
      },
    },
  },
  {
    title: String(adjustmentChromosome.value!.number),
    to: {
      name: 'adjustment-project_id-adjustment_run_id-generation_id-chromosome_id',
      params: {
        project_id: route.params.project_id,
        adjustment_run_id: route.params.adjustment_run_id,
        generation_id: route.params.generation_id,
        chromosome_id: route.params.chromosome_id,
      },
    },
  },
])

const fitness = computed(() =>
  new Intl.NumberFormat(userStore.locale, {
    minimumFractionDigits: 3,
    maximumFractionDigits: 3,
  }).format(adjustmentChromosome.value!.fitness)
)
</script>

<i18n locale="en-US" lang="json">
{
  "title": "Chromosome ({fitness})"
}
</i18n>

<i18n locale="ru-RU" lang="json">
{
  "title": "Хромосома ({fitness})"
}
</i18n>

<style lang="sass">
.chromosome-id__card-text
  height: calc(100% - 52px)
</style>
