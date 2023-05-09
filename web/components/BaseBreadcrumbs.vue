<template>
  <VContainer>
    <VBreadcrumbs color="blue-darken-1" class="px-0" :items="computedItems">
      <template #divider>
        <v-icon icon="mdi-chevron-right"></v-icon>
      </template>
    </VBreadcrumbs>
    <slot />
  </VContainer>
</template>

<script setup lang="ts">
import { BreadcrumbItem } from '~/types'
import { useI18n } from 'vue-i18n'

export interface Props {
  items?: BreadcrumbItem[]
}

const props = withDefaults(defineProps<Props>(), {
  items: () => [],
})

const { t } = useI18n({})

const computedItems = computed(() => {
  return [{ title: t('index'), to: { name: 'index' } }, ...props.items]
})
</script>

<i18n locale="en-US" lang="json">
{
  "index": "Homepage"
}
</i18n>

<i18n locale="ru-RU" lang="json">
{
  "index": "Главная"
}
</i18n>
