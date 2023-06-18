<template>
  <PluginsAdjustmentBreadcrumbs :items="bc" />
  <VCard height="calc(100% - 40px)">
    <VCardTitle class="d-flex align-center">
      {{ t('title', { time: adjustmentChromosome!.time, error }) }}
      <VSpacer />
      <PluginsAdjustmentSimulationIteration
        v-model="iteration"
        class="mr-2"
        :max-model-time="adjustmentRun!.maxModelTime"
      />
      <span class="mr-3 text-subtitle-1">{{
        t('iterationError', { error: iterationError })
      }}</span>
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
import { TimeSimulationExecutor } from 'fuzzy-cognitive-model-wasm'
import { useI18n } from 'vue-i18n'
import { useUserStore } from '~/store'
import { BreadcrumbsItem, Concept, Connection } from '~/types'

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
    { pick: ['name', 'modelCopyId', 'maxModelTime', 'dynamicModelType'] }
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

const getInitialConcepts = () => {
  return modelData.value!.concepts.map((concept) => {
    const newConcept = { ...concept }
    const conceptValue = adjustmentChromosome.value!.conceptValues.find(
      (conceptValue) => conceptValue.conceptId === concept.id
    )
    if (conceptValue) {
      newConcept.value = conceptValue.value
    }
    return newConcept
  })
}

const model = ref({
  project: { ...modelData.value!.project },
  concepts: getInitialConcepts(),
  connections: modelData.value!.connections.map((connection) => {
    const newConnection = { ...connection }
    const connectionValue = adjustmentChromosome.value!.connectionValues.find(
      (connectionValue) => connectionValue.connectionId === connection.id
    )
    if (connectionValue) {
      newConnection.value = connectionValue.value
    }
    return newConnection
  }),
})

const formatter = computed(() =>
  Intl.NumberFormat(userStore.locale, {
    minimumFractionDigits: 5,
  })
)

const iteration = ref(0)
const iterationError = ref('')
onMounted(() => {
  watch(
    iteration,
    (newValue) => {
      const concepts = modelData.value!.concepts.map<Concept>((concept) => {
        const conceptValue = adjustmentChromosome.value!.conceptValues.find(
          (conceptValue) => conceptValue.conceptId === concept.id
        )
        return {
          id: concept.id,
          value: conceptValue?.value || concept.value!,
          isControl: concept.pluginsData.controlConcepts!.isControl,
          isTarget: concept.pluginsData.targetConcepts!.isTarget,
          targetValue: concept.pluginsData.targetConcepts!.isTarget
            ? concept.pluginsData.targetConcepts!
            : null,
          constraint: concept.pluginsData.conceptConstraints!.hasConstraint
            ? concept.pluginsData.conceptConstraints!
            : null,
          dynamicModel: concept.pluginsData.adjustment!.dynamicModelType,
        }
      })
      const conceptsMap = new Map()
      for (const concept of concepts) {
        conceptsMap.set(concept.id, concept)
      }

      const connections = modelData.value!.connections.map<Connection>(
        (connection) => {
          const connectionValue =
            adjustmentChromosome.value!.connectionValues.find(
              (connectionValue) =>
                connectionValue.connectionId === connection.id
            )
          return {
            id: connection.id,
            value: connectionValue?.value || connection.value,
            sourceId: connection.sourceId,
            targetId: connection.targetId,
            isControl: connection.pluginsData.controlConnections!.isControl,
            constraint: connection.pluginsData.connectionConstraints!
              .hasConstraint
              ? connection.pluginsData.connectionConstraints!
              : null,
          }
        }
      )
      const connectionsMap = new Map()
      for (const connection of connections) {
        connectionsMap.set(connection.id, connection)
      }

      const conceptState = new Map()
      for (const concept of concepts) {
        conceptState.set(concept.id, concept.value)
      }

      const connectionState = new Map()
      for (const connection of connections.filter(
        (connection) => connection.isControl
      )) {
        connectionState.set(connection.id, connection.value)
      }

      const executor = new TimeSimulationExecutor(
        newValue,
        conceptsMap,
        connectionsMap,
        concepts.filter((concept) => concept.isTarget),
        adjustmentRun.value!.dynamicModelType,
        conceptState,
        connectionState
      )

      if (newValue === 0) {
        model.value.concepts = getInitialConcepts()
        iterationError.value = formatter.value.format(executor.get_error())
        return
      }

      while (executor.next()) {}
      for (const concept of model.value.concepts) {
        const state = executor.get_state() as Map<number, number>
        concept.value = state.get(concept.id)!
      }

      iterationError.value = formatter.value.format(executor.get_error())
    },
    { immediate: true }
  )
})

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

const error = computed(() =>
  formatter.value.format(adjustmentChromosome.value!.error)
)
</script>

<i18n locale="en-US" lang="json">
{
  "title": "Chromosome ({time}; {error})",
  "iterationError": "Error: {error}"
}
</i18n>

<i18n locale="ru-RU" lang="json">
{
  "title": "Хромосома ({time}; {error})",
  "iterationError": "Ошибка: {error}"
}
</i18n>

<style lang="sass">
.chromosome-id__card-text
  height: calc(100% - 52px)
</style>
