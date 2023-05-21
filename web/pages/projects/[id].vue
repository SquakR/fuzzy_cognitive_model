<template>
  <VProgressLinear v-if="loading" class="mt-2" color="primary" indeterminate />
  <VCard v-else height="100%">
    <VCardTitle>{{ model!.project.name }}</VCardTitle>
    <VCardText class="d-flex flex-column model-id__model-editor-container">
      <ModelEditor :model="model!"></ModelEditor>
    </VCardText>
  </VCard>
</template>

<script setup lang="ts">
definePageMeta({
  layout: 'editor',
  middleware: 'auth',
})
const route = useRoute()

const { data: model, pending: loading } = useGetModel(
  { key: 'model' },
  Number(route.params.id)
)
</script>

<style lang="sass">
.model-id__model-editor-container
  height: calc(100% - 48px)
</style>
