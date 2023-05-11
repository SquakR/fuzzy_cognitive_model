<template>
  <VLayout full-height>
    <TheAppBar />
    <VMain>
      <VContainer class="editor-layout__container" fluid>
        <NuxtPage />
      </VContainer>

      <TheGlobalMessage />
    </VMain>
  </VLayout>
</template>

<style lang="sass">
.editor-layout__container
  height: 100%
</style>
