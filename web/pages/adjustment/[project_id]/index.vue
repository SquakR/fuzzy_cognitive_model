<template>
  <VCard class="mt-2">
    <VCardTitle>{{ t('title') }}</VCardTitle>
    <VCardText>
      <VRow>
        <VCol cols="12">
          <PluginsAdjustmentAdjustForm
            :project-id="Number($route.params.project_id)"
            :adjust="adjust"
          >
            <template #activator="{ props }">
              <VBtn v-bind="props" color="primary">{{ t('adjust') }}</VBtn>
            </template>
          </PluginsAdjustmentAdjustForm>
        </VCol>
      </VRow>
      <VDataTableServer
        v-model:items-per-page="adjustmentRunsIn.perPage"
        v-model:page="adjustmentRunsIn.page"
        :headers="headers"
        :loading="adjustmentRunsPending"
        :items-length="adjustmentRunsItemsLength"
        :items="adjustmentRuns"
        show-expand
      >
        <template #item.name="{ item }">
          <template v-if="item.raw.resultChromosome">
            <NuxtLink :to="getGenerationLink(item.raw)">{{
              item.raw.name
            }}</NuxtLink>
          </template>
          <span v-else>{{ item.raw.name }}</span>
        </template>
        <template #item.description="{ item }">
          <span v-if="item.raw.description">{{ item.raw.description }}</span>
          <strong v-else>&mdash;</strong>
        </template>
        <template #item.createdAt="{ item }">
          {{ dateTimeFilter(item.raw.createdAt) }}
        </template>
        <template #item.result="{ item }">
          <template v-if="item.raw.resultChromosome">
            <NuxtLink :to="getResultChromosomeLink(item.raw)" class="mr-2">{{
              t('result')
            }}</NuxtLink>
            <PluginsAdjustmentResultChromosomeInfo :adjustment-run="item.raw" />
          </template>
          <VProgressLinear
            v-else
            color="primary"
            :model-value="
              (lastGenerations[item.raw.id] /
                item.raw.stopCondition.maxGenerations) *
              100
            "
          />
        </template>
        <template #expanded-row="{ item }">
          <tr>
            <td :colspan="2">{{ t('maxModelTime') }}</td>
            <td :colspan="3">{{ item.raw.maxModelTime }}</td>
          </tr>
          <tr class="adjustment__dynamic-model-type">
            <td :colspan="2">{{ t('dynamicModelType') }}</td>
            <td :colspan="3">
              <BaseMathJax :formula="getFormula(item.raw)" />
            </td>
          </tr>
          <tr>
            <td :colspan="2">{{ t('generationSize') }}</td>
            <td :colspan="3">{{ item.raw.generationSize }}</td>
          </tr>
          <tr>
            <td :colspan="2">{{ t('generationSaveInterval') }}</td>
            <td :colspan="3">{{ item.raw.generationSaveInterval }}</td>
          </tr>
          <tr>
            <td :colspan="2">{{ t('maxGenerations') }}</td>
            <td :colspan="3">{{ item.raw.stopCondition.maxGenerations }}</td>
          </tr>
          <tr>
            <td :colspan="2">{{ t('maxWithoutImprovements') }}</td>
            <td :colspan="3">
              {{ item.raw.stopCondition.maxWithoutImprovements }}
            </td>
          </tr>
          <tr>
            <td :colspan="2">{{ t('error') }}</td>
            <td :colspan="3">
              {{
                Intl.NumberFormat(userStore.locale).format(
                  item.raw.stopCondition.error
                )
              }}
            </td>
          </tr>
        </template>
      </VDataTableServer>
    </VCardText>
  </VCard>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { VDataTableServer } from 'vuetify/labs/VDataTable'
import { LocalAdjustmentRunsInType } from '~/composables/plugins/adjustment'
import { useUserStore } from '~/store'
import { AdjustmentRunOutType } from '~/types'

definePageMeta({
  middleware: 'auth',
})

const { t } = useI18n()
const route = useRoute()
const userStore = useUserStore()

const headers = computed(() => [
  { key: 'name', title: t('name'), sortable: false },
  { key: 'description', title: t('description'), sortable: false },
  { key: 'createdAt', title: t('createdAt'), sortable: false },
  { key: 'result', title: t('result'), sortable: false },
])
const dynamicModelTypes = {
  delta_delta: '\\(\\Delta K_j(t+1)=\\sum_{i=1}^Nw_{ij}\\Delta K_i(t)\\)',
  delta_value: '\\(\\Delta K_j(t+1)=\\sum_{i=1}^Nw_{ij}K_i(t)\\)',
  value_delta: '\\(K_j(t+1)=\\sum_{i=1}^Nw_{ij}\\Delta K_i(t)\\)',
  value_value: '\\(K_j(t+1)=\\sum_{i=1}^Nw_{ij}K_i(t)\\)',
}

const adjustmentRunsIn = ref<LocalAdjustmentRunsInType>({
  search: null,
  createdAtStart: null,
  createdAtIncludeStart: null,
  createdAtEnd: null,
  createdAtIncludeEnd: null,
  page: 1,
  perPage: 10,
})
const {
  adjustmentRuns,
  adjustmentRunsPending,
  adjustmentRunsItemsLength,
  lastGenerations,
  adjust,
} = await useAdjustmentRuns(Number(route.params.project_id), adjustmentRunsIn)

const getGenerationLink = (adjustmentRun: AdjustmentRunOutType) => {
  return {
    name: 'adjustment-project_id-adjustment_run_id',
    params: {
      project_id: route.params.project_id,
      adjustment_run_id: adjustmentRun.id,
    },
  }
}
const getResultChromosomeLink = (adjustmentRun: AdjustmentRunOutType) => {
  return {
    name: 'adjustment-project_id-adjustment_run_id-generation_id-chromosome_id',
    params: {
      project_id: route.params.project_id,
      adjustment_run_id: adjustmentRun.id,
      generation_id: adjustmentRun.resultChromosome!.generationId,
      chromosome_id: adjustmentRun.resultChromosome!.id,
    },
  }
}
const getFormula = (adjustmentRun: AdjustmentRunOutType) => {
  return dynamicModelTypes[adjustmentRun.dynamicModelType]
}
</script>

<i18n locale="en-US" lang="json">
{
  "title": "Adjustment With Genetic Algorithms",
  "adjust": "Adjust",
  "name": "Name",
  "description": "Description",
  "createdAt": "Date of creation",
  "result": "Result",
  "maxModelTime": "Maximum Model Time",
  "dynamicModelType": "Dynamic Model",
  "generationSize": "Generation Size",
  "generationSaveInterval": "Generation Save Interval",
  "maxGenerations": "Maximum Number Of Generations",
  "maxWithoutImprovements": "Maximum Number Of Generations Without Improvements",
  "error": "Error"
}
</i18n>

<i18n locale="ru-RU" lang="json">
{
  "title": "Структурно-параметрическая настройка",
  "adjust": "Настроить",
  "name": "Название",
  "description": "Описание",
  "createdAt": "Дата создание",
  "result": "Результат",
  "maxModelTime": "Максимальное модельное время",
  "dynamicModelType": "Модель динамики",
  "generationSize": "Размер поколения",
  "generationSaveInterval": "Интервал сохранения поколений",
  "maxGenerations": "Максимальное число поколений",
  "maxWithoutImprovements": "Максимальное число поколений без улучшений",
  "error": "Ошибка"
}
</i18n>

<style lang="sass">
.adjustment__dynamic-model-type
  .MathJax
    font-size: 1.1rem !important
</style>
