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
import { BreadcrumbItem } from '~/types/base-breadcrumbs'
import { useI18n } from 'vue-i18n'

export interface Props {
  items: BreadcrumbItem[]
}

const props = defineProps<Props>()

const { t } = useI18n({})

const computedItems = computed(() => {
  if (props.items[0].to === '/') {
    return props.items
  }
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
