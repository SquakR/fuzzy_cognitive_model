<template>
  <BaseModalForm
    v-model="isActive"
    :action-key="CREATE_CONCEPT_KEY"
    :title="t('title')"
    :button-text="t('buttonText')"
    :validation-schema="validationSchema"
    :initial-values="initialValues"
    :on-submit="onSubmit"
  >
    <BaseTextField :label="t('name')" name="name" />
    <BaseTextarea :label="t('description')" name="description" />
    <BaseTextField :label="t('value')" name="value" />
    <BaseTextField :label="t('xPosition')" name="xPosition" />
    <BaseTextField :label="t('yPosition')" name="yPosition" />
  </BaseModalForm>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import * as yup from 'yup'
import { useUserStore } from '~/store'
import { CREATE_CONCEPT_KEY, ConceptInCreateType, ModelOutType } from '~/types'

export interface Props {
  modelValue: boolean
  model: ModelOutType
  xPosition: number
  yPosition: number
}
export interface Emits {
  (e: 'update:modelValue', modelValue: boolean): void
  (e: 'createConcept', conceptIn: ConceptInCreateType): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const { $yup } = useNuxtApp()
const userStore = useUserStore()
const { t } = useI18n()

const isActive = computed({
  get: () => props.modelValue,
  set: (value) => {
    emit('update:modelValue', value)
  },
})

const validationSchema = computed(() => {
  const validationSchema: yup.ObjectShape = {
    name: $yup.string().required().min(1).max(255),
    description: $yup.string(),
    xPosition: $yup.number().required(),
    yPosition: $yup.number().required(),
  }
  if (props.model.project.conceptValueType === 'from_zero_to_one') {
    validationSchema.value = $yup.number().required().min(0).max(1)
  }
  return $yup.object(validationSchema)
})
const initialValues = computed(() => {
  const initialValues: Record<string, string> = {
    name: '',
    description: '',
    xPosition: String(props.xPosition),
    yPosition: String(props.yPosition),
  }
  if (props.model.project.conceptValueType === 'from_zero_to_one') {
    initialValues.value = userStore.locale === 'ru-RU' ? '0,0' : '0.0'
  }
  return initialValues
})
const onSubmit = async (values: Record<string, string>) => {
  emit('createConcept', {
    name: values.name,
    description: values.description,
    value: values.value ? Number(values.value) : null,
    xPosition: Number(values.xPosition),
    yPosition: Number(values.yPosition),
  })
}
</script>

<i18n locale="en-US" lang="json">
{
  "title": "Add Concept",
  "buttonText": "Add",
  "name": "Name",
  "description": "Description",
  "value": "Value",
  "xPosition": "Position x",
  "yPosition": "Position y"
}
</i18n>

<i18n locale="ru-RU" lang="json">
{
  "title": "Добавление концепта",
  "buttonText": "Добавить",
  "name": "Название",
  "description": "Описание",
  "value": "Значение",
  "xPosition": "Позиция x",
  "yPosition": "Позиция y"
}
</i18n>
