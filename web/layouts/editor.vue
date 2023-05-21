<template>
  <VLayout full-height>
    <TheAppBar />
    <VMain>
      <VContainer class="editor-layout_full-height" fluid>
        <NuxtPage />
      </VContainer>
      <TheGlobalMessage />
    </VMain>
    <div class="editor-layout__right-menu"></div>
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
.editor-layout__right-menu
  pointer-events: none
  position: absolute
  top: 64px
  width: 100%
  height: calc(100% - 64px)
</style>
