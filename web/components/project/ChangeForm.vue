<template>
  <BaseModalForm
    ref="modelForm"
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
    <BaseTextarea :label="t('description')" name="description" />
    <BaseCheckbox :label="t('isPublic')" name="isPublic" />
    <BaseCheckbox :label="t('isArchived')" name="isArchived" />
  </BaseModalForm>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import * as yup from 'yup'
import { ProjectOutType } from '~/types'
import BaseModalForm from '~/components/base/ModalForm.vue'

export interface Props {
  modelValue: ProjectOutType | null
}
export interface Emits {
  (e: 'update:modelValue', modelValue: ProjectOutType | null): void
  (e: 'changeProject', project: ProjectOutType): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const modelForm = ref<InstanceType<typeof BaseModalForm> | null>(null)

const { t } = useI18n()

const isActive = computed({
  get: () => Boolean(props.modelValue),
  set: (value) => {
    if (!value) {
      emit('update:modelValue', null)
    }
  },
})

const actionKey = 'changeProject'
const validationSchema = yup.object({
  name: yup.string().required().min(3).max(255),
  description: yup.string().required(),
  isPublic: yup.boolean().required(),
  isArchived: yup.boolean().required(),
})
const initialValues: Ref<yup.InferType<typeof validationSchema>> = ref({
  name: '',
  description: '',
  isPublic: false,
  isArchived: false,
})

watch(
  () => props.modelValue,
  async (newValue) => {
    if (newValue) {
      await nextTick()
      modelForm.value?.form?.setValues({
        name: newValue.name,
        description: newValue.description,
        isPublic: newValue.isPublic,
        isArchived: newValue.isArchived,
      })
    }
  }
)

const { execute: changeProject, onSuccess } = useChangeProject({
  key: actionKey,
})
onSuccess((project) => {
  emit('changeProject', project)
  isActive.value = false
})

const onSubmit = async (values: yup.InferType<typeof validationSchema>) => {
  await changeProject(props.modelValue!.id, {
    name: values.name,
    description: values.description,
    isPublic: values.isPublic,
    isArchived: values.isArchived,
    conceptValueType: 'from_zero_to_one',
    connectionValueType: 'from_minus_one_to_one',
  })
}
</script>

<i18n locale="en-US" lang="json">
{
  "title": "Change Project",
  "buttonText": "Change",
  "name": "Name",
  "description": "Description",
  "isPublic": "Public",
  "isArchived": "Archived"
}
</i18n>

<i18n locale="ru-RU" lang="json">
{
  "title": "Изменение проекта",
  "buttonText": "Изменить",
  "name": "Название",
  "description": "Описание",
  "isPublic": "Открытый",
  "isArchived": "Архивированный"
}
</i18n>
