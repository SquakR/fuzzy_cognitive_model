<template>
  <VCard height="100%">
    <VCardTitle>{{ model!.project.name }}</VCardTitle>
    <VCardText class="d-flex flex-column project-id__card-text">
      <ModelEditor :model="model!"></ModelEditor>
    </VCardText>
  </VCard>
</template>

<script setup lang="ts">
definePageMeta({
  layout: 'model',
  middleware: 'auth',
})
const route = useRoute()

const { data: model } = await useGetModel(
  { key: 'model' },
  Number(route.params.id)
)
</script>

<style lang="sass">
.project-id__card-text
  height: calc(100% - 48px)
</style>
