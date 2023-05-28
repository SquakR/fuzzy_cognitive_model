<template>
  <PluginsAdjustmentBreadcrumbs :items="bc" />
  <VCard>
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
        :items="adjustmentGenerations"
      >
        <template #item.number="{ item }">
          <NuxtLink :to="getChromosomesLink(item.raw)">
            {{ item.raw.number }}
          </NuxtLink>
        </template>
        <template #item.error="{ item }">
          {{ formatGenerationError(item.raw) }}
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
import { AdjustmentGenerationOutType, BreadcrumbsItem } from '~/types'

definePageMeta({
  middleware: 'auth',
})

const { t } = useI18n()
const route = useRoute()
const userStore = useUserStore()

const headers = computed(() => [
  { key: 'number', title: t('number'), sortable: false },
  { key: 'error', title: t('error'), sortable: false },
  { key: 'fitness', title: t('fitness'), sortable: false },
])

const page = ref(1)
const perPage = ref(10)

const [
  { data: project },
  { data: adjustmentRun },
  { data: adjustmentGenerationPagination, pending: loading, refresh },
] = await Promise.all([
  useGetProject({ key: 'project' }, Number(route.params.project_id), {
    pick: ['name'],
  }),
  useGetAdjustmentRun(
    { key: 'adjustmentRun' },
    Number(route.params.adjustment_run_id),
    { pick: ['name'] }
  ),
  useGetAdjustmentGenerations(
    { key: 'adjustmentGenerations' },
    Number(route.params.adjustment_run_id),
    page,
    perPage
  ),
])

const { itemsLength, data: adjustmentGenerations } = usePagination(
  adjustmentGenerationPagination,
  refresh,
  page,
  perPage
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
])

const getChromosomesLink = (
  adjustmentGeneration: AdjustmentGenerationOutType
) => {
  return {
    name: 'adjustment-project_id-adjustment_run_id-generation_id',
    params: {
      project_id: route.params.project_id,
      adjustment_run_id: route.params.adjustment_run_id,
      generation_id: adjustmentGeneration.id,
    },
  }
}

const formatter = new Intl.NumberFormat(userStore.locale, {
  minimumFractionDigits: 5,
})

const formatGenerationError = (
  adjustmentGeneration: AdjustmentGenerationOutType
) => {
  return formatter.format(adjustmentGeneration.error)
}
const formatGenerationFitness = (
  adjustmentGeneration: AdjustmentGenerationOutType
) => {
  return formatter.format(adjustmentGeneration.fitness)
}
</script>

<i18n locale="en-US" lang="json">
{
  "title": "Generations",
  "number": "Number",
  "error": "Error",
  "fitness": "Fitness"
}
</i18n>

<i18n locale="ru-RU" lang="json">
{
  "title": "Поколения",
  "number": "Номер",
  "error": "Ошибка",
  "fitness": "Приспособленность"
}
</i18n>
