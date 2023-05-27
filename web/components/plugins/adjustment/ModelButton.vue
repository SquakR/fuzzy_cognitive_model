<template>
  <VBtn
    color="primary"
    :to="{ name: 'projects-id', params: { id: projectId } }"
    >{{ t('model') }}</VBtn
  >
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'

export interface Props {
  projectId: number
}

defineProps<Props>()

const { t } = useI18n()
</script>

<i18n locale="en-US" lang="json">
{
  "model": "Model"
}
</i18n>

<i18n locale="ru-RU" lang="json">
{
  "model": "Модель"
}
</i18n>
