<template>
  <div class="layout__container_default">
    <TheHeader />
    <main class="layout__main_default">
      <slot />
    </main>
    <TheFooter />
  </div>
</template>

<style lang="sass">
.layout__container_default
  display: flex
  flex-direction: column
  height: 100vh

.layout__main_default
  flex-grow: 1
</style>
