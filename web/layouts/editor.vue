<template>
  <VLayout full-height>
    <TheAppBar />
    <VMain>
      <VContainer class="editor-layout_full-height" fluid>
        <NuxtPage />
      </VContainer>
      <TheGlobalMessage />
    </VMain>
    <div class="editor-layout__drawer"></div>
  </VLayout>
</template>

<script setup lang="ts">
onMounted(() => {
  document.documentElement.classList.add(
    'editor-layout__html',
    'editor-layout_full-height'
  )
  document.body.classList.add('editor-layout_full-height')
  document.querySelector('#__nuxt')!.classList.add('editor-layout_full-height')
})
onUnmounted(() => {
  document.documentElement.classList.remove(
    'editor-layout__html',
    'editor-layout_full-height'
  )
  document.body.classList.remove('editor-layout_full-height')
  document
    .querySelector('#__nuxt')!
    .classList.remove('editor-layout_full-height')
})
</script>

<style lang="sass">
.editor-layout__html
  overflow: hidden
.editor-layout_full-height
  height: 100%
.editor-layout__drawer
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
