<template>
  <div class="flex h-screen flex-col">
    <TheHeader />
    <main class="grow pt-14">
      <slot />
    </main>
    <TheFooter />
  </div>
</template>
