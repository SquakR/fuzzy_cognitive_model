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
            <VListItemTitle>{{
              t('time', { time: adjustmentRun.resultIndividual!.time })
            }}</VListItemTitle>
          </VListItem>
          <VListItem>
            <VListItemTitle>{{ t('error', { error }) }}</VListItemTitle>
          </VListItem>
          <VListItem>
            <VListItemTitle>{{
              t('generationNumber', { generationNumber })
            }}</VListItemTitle>
          </VListItem>
          <VListItem>
            <VListItemTitle>{{
              t('generationError', { generationError })
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

const formatter = computed(
  () =>
    new Intl.NumberFormat(userStore.locale, {
      minimumFractionDigits: 5,
    })
)
const error = computed(() =>
  formatter.value.format(props.adjustmentRun.resultIndividual!.error)
)
const generationNumber = computed(
  () => props.adjustmentRun.resultIndividual!.generationNumber
)
const generationError = computed(() =>
  formatter.value.format(props.adjustmentRun.resultIndividual!.generationError)
)
const generationLink = computed(() => ({
  name: 'adjustment-project_id-adjustment_run_id-generation_id',
  params: {
    project_id: route.params.project_id,
    adjustment_run_id: props.adjustmentRun.id,
    generation_id: props.adjustmentRun.resultIndividual!.generationId,
  },
}))
</script>

<i18n locale="en-US" lang="json">
{
  "data": "Data",
  "time": "Time: {time}",
  "error": "Error: {error}",
  "generationNumber": "Generation number: {generationNumber}",
  "generationError": "Generation error: {generationError}",
  "generation": "Generation"
}
</i18n>

<i18n locale="ru-RU" lang="json">
{
  "data": "Данные",
  "time": "Время: {time}",
  "error": "Ошибка: {error}",
  "generationNumber": "Номер поколения: {generationNumber}",
  "generationError": "Ошибка поколения: {generationError}",
  "generation": "Поколение"
}
</i18n>
