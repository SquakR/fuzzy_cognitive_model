<template>
  <div></div>
</template>

<script setup lang="ts">
import { useUserStore } from '~/store'

definePageMeta({
  middleware: [
    () => {
      const userStore = useUserStore()
      if (userStore.user) {
        return navigateTo({ name: 'projects' })
      } else {
        return navigateTo({ name: 'auth-sign_in' })
      }
    },
  ],
})
</script>
