<template>
  <VLayout full-height>
    <TheAppBar />
    <VMain>
      <VContainer class="editor-layout__container" fluid>
        <NuxtPage />
      </VContainer>
      <TheGlobalMessage />
    </VMain>
    <div class="editor-layout__right-menu"></div>
  </VLayout>
</template>

<style lang="sass">
.editor-layout__container
  height: 100%
.editor-layout__right-menu
  pointer-events: none
  position: absolute
  top: 64px
  width: 100%
  height: calc(100% - 64px)
</style>
