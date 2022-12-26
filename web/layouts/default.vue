<template>
  <div class="layout__container_default">
    <TheHeader />
    <main class="layout__main_default">
      <div class="container is-fluid layout__main_container">
        <slot />
      </div>
    </main>
    <TheFooter />
    <TheGlobalMessage />
  </div>
</template>

<style lang="sass">
@import "~~/assets/styles/variables.sass"

.layout__container_default
  display: flex
  flex-direction: column
  height: 100vh

.layout__main_default
  flex-grow: 1

.layout__main_container
  padding-top: $navbar-height
</style>
