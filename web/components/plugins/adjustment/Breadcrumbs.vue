<template>
  <VBreadcrumbs :items="items" density="compact" color="indigo-accent-2">
    <template #divider>
      <VIcon icon="mdi-chevron-right"></VIcon>
    </template>
  </VBreadcrumbs>
</template>

<script setup lang="ts">
import { BreadcrumbsItem } from '~/types'

export interface Props {
  items: BreadcrumbsItem[]
}
defineProps<Props>()
</script>
