<template>
  <BaseModalForm
    v-model="isActive"
    :action-key="actionKey"
    :title="t('title')"
    :button-text="t('buttonText')"
    :validation-schema="validationSchema"
    :initial-values="initialValues"
    :on-submit="onSubmit"
  >
    <template #activator="{ props }">
      <slot name="activator" :props="props" />
    </template>
    <BaseTextField :label="t('name')" name="name" />
    <BaseTextArea :label="t('description')" name="description" />
    <BaseCheckbox :label="t('isPublic')" name="isPublic" />
  </BaseModalForm>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import * as yup from 'yup'
import { ProjectOutType } from '~/types'

export interface Emits {
  (e: 'addProject', project: ProjectOutType): void
}
const emit = defineEmits<Emits>()

const { t } = useI18n({})

const isActive = ref(false)
const actionKey = 'createProject'
const validationSchema = yup.object({
  name: yup.string().required().min(3).max(255),
  description: yup.string().required(),
  isPublic: yup.boolean().required(),
})
const initialValues: yup.InferType<typeof validationSchema> = {
  name: '',
  description: '',
  isPublic: false,
}
const createProject = useCreateProject({
  key: actionKey,
  onSuccess: (project) => {
    emit('addProject', project)
    isActive.value = false
  },
})
const onSubmit = async (values: yup.InferType<typeof validationSchema>) => {
  await createProject({
    name: values.name,
    description: values.description,
    isPublic: values.isPublic,
    isArchived: false,
    conceptValueType: 'from_zero_to_one',
    connectionValueType: 'from_minus_one_to_one',
  })
}
</script>

<i18n locale="en-US" lang="json">
{
  "title": "Add Project",
  "buttonText": "Add",
  "name": "Name",
  "description": "Description",
  "isPublic": "Public"
}
</i18n>

<i18n locale="ru-RU" lang="json">
{
  "title": "Добавление проекта",
  "buttonText": "Добавить",
  "name": "Название",
  "description": "Описание",
  "isPublic": "Открытый"
}
</i18n>
