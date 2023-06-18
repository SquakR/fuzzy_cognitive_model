<template>
  <BaseModalForm
    v-model="isActive"
    :action-key="ADJUST_KEY"
    :title="t('title')"
    :button-text="t('buttonText')"
    :validation-schema="validationSchema"
    :initial-values="initialValues"
    :on-submit="onSubmit"
    height="60vh"
  >
    <template #activator="{ props }">
      <slot name="activator" :props="props" />
    </template>
    <BaseTextField :label="t('name')" name="name" />
    <BaseTextarea :label="t('description')" name="description" />
    <BaseTextField :label="t('minModelTime')" name="minModelTime" />
    <BaseTextField :label="t('maxModelTime')" name="maxModelTime" />
    <PluginsAdjustmentDynamicModelTypeSelect />
    <BaseTextField :label="t('generationSize')" name="generationSize" />
    <BaseTextField
      :label="t('generationSaveInterval')"
      name="generationSaveInterval"
    />
    <BaseTextField :label="t('maxGenerations')" name="maxGenerations" />
    <BaseTextField
      :label="t('maxWithoutImprovements')"
      name="maxWithoutImprovements"
    />
    <BaseTextField :label="t('error')" name="error" />
  </BaseModalForm>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { useUserStore } from '~/store'
import { ADJUST_KEY, DynamicModelType } from '~/types'

export interface Props {
  projectId: number
  adjust: Awaited<ReturnType<typeof useAdjustmentRuns>>['adjust']
}

const props = defineProps<Props>()

interface Values {
  name: string
  description: string
  minModelTime: string
  maxModelTime: string
  dynamicModelType: DynamicModelType
  generationSize: string
  generationSaveInterval: string
  maxGenerations: string
  maxWithoutImprovements: string
  error: string
}

const { $yup } = useNuxtApp()
const { t } = useI18n()
const userStore = useUserStore()

const isActive = ref(false)
const validationSchema = $yup.object({
  name: $yup.string().required().min(3).max(255),
  description: $yup.string(),
  maxModelTime: $yup.number().integer().min(5).max(10000),
  dynamicModelType: $yup
    .string()
    .notRequired()
    .oneOf(['delta_delta', 'delta_value', 'value_delta', 'value_value']),
  generationSize: $yup.number().integer().min(10).max(10000),
  generationSaveInterval: $yup
    .number()
    .integer()
    .min(1)
    .when('generationSize', ([generationSize], schema) =>
      schema.max(generationSize)
    ),
  maxGenerations: $yup.number().integer().min(10).max(10000),
  maxWithoutImprovements: $yup.number().integer().min(10).max(10000),
  error: $yup.number().moreThan(0),
})
const initialValues = computed<Values>(() => ({
  name: '',
  description: '',
  minModelTime: '0',
  maxModelTime: '100',
  dynamicModelType: 'delta_delta',
  generationSize: '100',
  generationSaveInterval: '10',
  maxGenerations: '1000',
  maxWithoutImprovements: '10',
  error: new Intl.NumberFormat(userStore.locale).format(0.001),
}))

const onSubmit = async (values: Values) => {
  await props.adjust(props.projectId, {
    name: values.name,
    description: values.description,
    minModelTime: Number(values.minModelTime),
    maxModelTime: Number(values.maxModelTime),
    dynamicModelType: values.dynamicModelType,
    generationSize: Number(values.generationSize),
    generationSaveInterval: Number(values.generationSaveInterval),
    stopCondition: {
      maxGenerations: Number(values.maxGenerations),
      maxWithoutImprovements: Number(values.maxWithoutImprovements),
      error: Number(values.error.replace(',', '.')),
    },
  })
  isActive.value = false
}
</script>

<i18n locale="en-US" lang="json">
{
  "title": "Adjustment",
  "buttonText": "Adjust",
  "name": "Name",
  "description": "Description",
  "minModelTime": "Minimum Model Time",
  "maxModelTime": "Maximum Model Time",
  "generationSize": "Generation Size",
  "generationSaveInterval": "Generation Save Interval",
  "maxGenerations": "Maximum Number Of Generations",
  "maxWithoutImprovements": "Maximum Number Of Generations Without Improvements",
  "error": "Error"
}
</i18n>

<i18n locale="ru-RU" lang="json">
{
  "title": "Настройка",
  "buttonText": "Настроить",
  "name": "Название",
  "description": "Описание",
  "minModelTime": "Минимальное модельное время",
  "maxModelTime": "Максимальное модельное время",
  "generationSize": "Размер поколения",
  "generationSaveInterval": "Интервал сохранения поколений",
  "maxGenerations": "Максимальное число поколений",
  "maxWithoutImprovements": "Максимальное число поколений без улучшений",
  "error": "Ошибка"
}
</i18n>
