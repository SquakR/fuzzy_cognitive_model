<template>
  <VTooltip v-model="tooltipActive" location="bottom">
    <template #activator="{ props: tooltipProps }">
      <VMenu>
        <template #activator="{ props: menuProps }">
          <VIcon
            v-bind="{ ...tooltipProps, ...menuProps }"
            @click="tooltipActive = false"
            >mdi-information-outline</VIcon
          >
        </template>
        <VList>
          <VListItem>
            <VListItemTitle>{{ t('fitness', { fitness }) }}</VListItemTitle>
          </VListItem>
          <VListItem>
            <VListItemTitle>{{
              t('generationNumber', { generationNumber })
            }}</VListItemTitle>
          </VListItem>
          <VListItem>
            <VListItemTitle>{{
              t('generationFitness', { generationFitness })
            }}</VListItemTitle>
          </VListItem>
          <VListItem>
            <VListItemTitle>
              <NuxtLink :to="generationLink">{{ t('generation') }}</NuxtLink>
            </VListItemTitle>
          </VListItem>
        </VList>
      </VMenu>
    </template>
    <span>{{ t('data') }}</span>
  </VTooltip>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { useUserStore } from '~/store'
import { AdjustmentRunOutType } from '~/types'

export interface Props {
  adjustmentRun: AdjustmentRunOutType
}

const props = defineProps<Props>()

const { t } = useI18n()
const route = useRoute()
const userStore = useUserStore()

const tooltipActive = ref(false)

const fitness = computed(() =>
  new Intl.NumberFormat(userStore.locale).format(
    props.adjustmentRun.resultChromosome!.fitness
  )
)
const generationNumber = computed(
  () => props.adjustmentRun.resultChromosome!.generationNumber
)
const generationFitness = computed(() =>
  Intl.NumberFormat(userStore.locale).format(
    props.adjustmentRun.resultChromosome!.generationFitness
  )
)
const generationLink = computed(() => ({
  name: 'adjustment-project_id-adjustment_run_id-generation_id',
  params: {
    project_id: route.params.project_id,
    adjustment_run_id: props.adjustmentRun.id,
    generation_id: props.adjustmentRun.resultChromosome!.generationId,
  },
}))
</script>

<i18n locale="en-US" lang="json">
{
  "data": "Data",
  "fitness": "Fitness: %{fitness}",
  "generationNumber": "Generation number: %{generationNumber}",
  "generationFitness": "Generation fitness: %{generationFitness}",
  "generation": "Generation"
}
</i18n>

<i18n locale="ru-RU" lang="json">
{
  "data": "Данные",
  "fitness": "Приспособленность: %{fitness}",
  "generationNumber": "Номер поколения: %{generationNumber}",
  "generationFitness": "Приспособленность поколения: %{generationFitness}",
  "generation": "Поколение"
}
</i18n>
