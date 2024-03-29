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
        :items="adjustmentIndividuals"
      >
        <template #item.number="{ item }">
          <NuxtLink :to="getIndividualLink(item.raw)">
            {{ item.raw.number }}
          </NuxtLink>
        </template>
        <template #item.error="{ item }">
          {{ formatGenerationError(item.raw) }}
        </template>
      </VDataTableServer>
    </VCardText>
  </VCard>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { VDataTableServer } from 'vuetify/labs/VDataTable'
import { useUserStore } from '~/store'
import { AdjustmentIndividualOutType, BreadcrumbsItem } from '~/types'

definePageMeta({
  middleware: 'auth',
})

const { t } = useI18n()
const route = useRoute()
const userStore = useUserStore()

const headers = computed(() => [
  { key: 'number', title: t('number'), sortable: false },
  { key: 'error', title: t('error'), sortable: false },
])

const page = ref(1)
const perPage = ref(10)

const [
  { data: project },
  { data: adjustmentRun },
  { data: adjustmentGeneration },
  { data: adjustmentIndividualPagination, pending: loading, refresh },
] = await Promise.all([
  useGetProject({ key: 'project' }, Number(route.params.project_id), {
    pick: ['name'],
  }),
  useGetAdjustmentRun(
    { key: 'adjustmentRun' },
    Number(route.params.adjustment_run_id),
    { pick: ['name'] }
  ),
  useGetAdjustmentGeneration(
    {
      key: 'adjustmentGeneration',
    },
    Number(route.params.generation_id),
    { pick: ['number'] }
  ),
  useGetAdjustmentIndividuals(
    { key: 'adjustmentIndividuals' },
    Number(route.params.generation_id),
    page,
    perPage
  ),
])

const { itemsLength, data: adjustmentIndividuals } = usePagination(
  adjustmentIndividualPagination,
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

const getIndividualLink = (
  adjustmentIndividual: AdjustmentIndividualOutType
) => {
  return {
    name: 'adjustment-project_id-adjustment_run_id-generation_id-individual_id',
    params: {
      project_id: route.params.project_id,
      adjustment_run_id: route.params.adjustment_run_id,
      generation_id: route.params.generation_id,
      individual_id: adjustmentIndividual.id,
    },
  }
}

const formatGenerationError = (
  adjustmentIndividual: AdjustmentIndividualOutType
) => {
  return new Intl.NumberFormat(userStore.locale, {
    minimumFractionDigits: 5,
  }).format(adjustmentIndividual.error)
}
</script>

<i18n locale="en-US" lang="json">
{
  "title": "Individuals",
  "number": "Number",
  "error": "Error"
}
</i18n>

<i18n locale="ru-RU" lang="json">
{
  "title": "Особи",
  "number": "Номер",
  "error": "Ошибка"
}
</i18n>
