<template>
  <PluginsAdjustmentBreadcrumbs :items="bc" />
  <VCard class="mt-2">
    <VCardTitle>{{ t('title') }}</VCardTitle>
    <VCardText>
      <VRow>
        <VCol class="d-flex" cols="12">
          <VSpacer />
          <PluginsAdjustmentModelButton
            :project-id="Number($route.params.project_id)"
          />
        </VCol>
      </VRow>
      <VDataTableServer
        v-model:items-per-page="perPage"
        v-model:page="page"
        :headers="headers"
        :loading="loading"
        :items-length="itemsLength"
        :items="adjustmentChromosomes"
      >
        <template #item.number="{ item }">
          <NuxtLink :to="getChromosomeLink(item.raw)">
            {{ item.raw.number }}
          </NuxtLink>
        </template>
        <template #item.fitness="{ item }">
          {{ formatGenerationFitness(item.raw) }}
        </template>
      </VDataTableServer>
    </VCardText>
  </VCard>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { VDataTableServer } from 'vuetify/labs/VDataTable'
import { useUserStore } from '~/store'
import { AdjustmentChromosomeOutType, BreadcrumbsItem } from '~/types'

definePageMeta({
  middleware: 'auth',
})

const { t } = useI18n()
const route = useRoute()
const userStore = useUserStore()

const headers = computed(() => [
  { key: 'number', title: t('number'), sortable: false },
  { key: 'fitness', title: t('fitness'), sortable: false },
])

const { data: project } = await useGetProject(
  { key: 'project' },
  Number(route.params.project_id),
  { pick: ['name'] }
)
const { data: adjustmentRun } = await useGetAdjustmentRun(
  { key: 'adjustmentRun' },
  Number(route.params.adjustment_run_id),
  { pick: ['name'] }
)
const { data: adjustmentGeneration } = await useGetAdjustmentGeneration(
  {
    key: 'adjustmentGeneration',
  },
  Number(route.params.generation_id),
  { pick: ['number'] }
)

const page = ref(1)
const perPage = ref(10)
const {
  data: adjustmentChromosomePagination,
  pending: loading,
  refresh,
} = await useGetAdjustmentChromosomes(
  { key: 'adjustmentChromosomes' },
  Number(route.params.generation_id),
  page,
  perPage
)
const { itemsLength, data: adjustmentChromosomesData } = usePagination(
  adjustmentChromosomePagination,
  refresh,
  page,
  perPage
)
const adjustmentChromosomes = computed(() =>
  adjustmentChromosomesData.value
    ? adjustmentChromosomesData.value.map((adjustmentChromosome, index) => ({
        ...adjustmentChromosome,
        number: perPage.value * (page.value - 1) + index + 1,
      }))
    : []
)

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
])

const getChromosomeLink = (
  adjustmentChromosome: AdjustmentChromosomeOutType
) => {
  return {
    name: 'adjustment-project_id-adjustment_run_id-generation_id-chromosome_id',
    params: {
      project_id: route.params.project_id,
      adjustment_run_id: route.params.adjustment_run_id,
      generation_id: route.params.generation_id,
      chromosome_id: adjustmentChromosome.id,
    },
  }
}
const formatGenerationFitness = (
  adjustmentChromosome: AdjustmentChromosomeOutType
) => {
  return new Intl.NumberFormat(userStore.locale, {
    minimumFractionDigits: 3,
    maximumFractionDigits: 3,
  }).format(adjustmentChromosome.fitness)
}
</script>

<i18n locale="en-US" lang="json">
{
  "title": "Chromosomes",
  "number": "Number",
  "fitness": "Fitness"
}
</i18n>

<i18n locale="ru-RU" lang="json">
{
  "title": "Хромосомы",
  "number": "Номер",
  "fitness": "Приспособленность"
}
</i18n>
