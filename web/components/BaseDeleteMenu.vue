<template>
  <VMenu>
    <template #activator="{ props }">
      <slot name="activator" :props="props" />
    </template>
    <VCard>
      <VCardTitle class="text-subtitle-1">{{
        t('title', { itemName })
      }}</VCardTitle>
      <VCardActions>
        <VBtn variant="elevated" color="error" @click="$emit('confirm')">{{
          t('yes')
        }}</VBtn>
        <VSpacer />
        <VBtn variant="elevated" color="success" @click="$emit('cancel')">{{
          t('no')
        }}</VBtn>
      </VCardActions>
    </VCard>
  </VMenu>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'

export interface Emits {
  (e: 'confirm'): void
  (e: 'cancel'): void
}
export interface Props {
  itemName: string
}

defineProps<Props>()
defineEmits<Emits>()

const { t } = useI18n()
</script>

<i18n locale="en-US" lang="json">
{
  "title": "Are you sure you want to delete the {itemName}?",
  "yes": "Yes",
  "no": "No"
}
</i18n>

<i18n locale="ru-RU" lang="json">
{
  "title": "Вы действительно хотите удалить {itemName}?",
  "yes": "Да",
  "no": "Нет"
}
</i18n>
