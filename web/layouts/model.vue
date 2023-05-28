<template>
  <VLayout full-height>
    <TheAppBar />
    <VMain>
      <VContainer class="model-layout_full-height" fluid>
        <NuxtPage />
      </VContainer>
      <TheGlobalMessage />
    </VMain>
    <div class="model-layout__drawer"></div>
  </VLayout>
</template>

<script setup lang="ts">
onMounted(() => {
  document.documentElement.classList.add(
    'model-layout__html',
    'model-layout_full-height'
  )
  document.body.classList.add('model-layout_full-height')
  document.querySelector('#__nuxt')!.classList.add('model-layout_full-height')
})
onUnmounted(() => {
  document.documentElement.classList.remove(
    'model-layout__html',
    'model-layout_full-height'
  )
  document.body.classList.remove('model-layout_full-height')
  document
    .querySelector('#__nuxt')!
    .classList.remove('model-layout_full-height')
})
</script>

<style lang="sass">
$grey-lighten-5: #FAFAFA

.model-layout__html
  overflow: hidden
.model-layout_full-height
  height: 100%

.model__cytoscape-container
  width: 100%
  background-color: $grey-lighten-5

.model-layout__drawer
  pointer-events: none
  position: absolute
  top: 64px
  width: 100%
  height: calc(100% - 64px)

  .drawer-card
    pointer-events: auto
    position: absolute
    height: 100%
    right: 0

  .drawer-card-text
    height: calc(100% - 48px)
    min-width: 500px
    overflow-y: auto
</style>
