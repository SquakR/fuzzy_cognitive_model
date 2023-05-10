<template>
  <VTooltip v-model="tooltipActive" location="bottom">
    <template #activator="{ props: tooltipProps }">
      <MenuDelete
        :item-name="itemName"
        @confirm="$emit('confirm')"
        @cancel="$emit('cancel')"
      >
        <template #activator="{ props: menuProps }">
          <VIcon
            v-bind="{ ...tooltipProps, ...menuProps }"
            color="error"
            @click="tooltipActive = false"
          >
            mdi-delete</VIcon
          >
        </template>
      </MenuDelete>
    </template>
    <span>{{ t('delete') }}</span>
  </VTooltip>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'

export interface Props {
  itemName: string
}
export interface Emits {
  (e: 'confirm'): void
  (e: 'cancel'): void
}

defineProps<Props>()
defineEmits<Emits>()

const { t } = useI18n()

const tooltipActive = ref(false)
</script>

<i18n locale="en-US" lang="json">
{
  "delete": "Delete"
}
</i18n>

<i18n locale="ru-RU" lang="json">
{
  "delete": "Удалить"
}
</i18n>
