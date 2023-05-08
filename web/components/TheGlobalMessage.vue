<template>
  <VSnackbar v-model="isActive" :color="color" :timeout="-1">
    {{ message }}
    <template #actions>
      <VBtn icon="mdi-close" size="x-small" @click="isActive = false" />
    </template>
  </VSnackbar>
</template>

<script setup lang="ts">
import { useMessageStore } from '~/store'

const messageStore = useMessageStore()
const { success, error, unsubscribe } = messageStore.subscribeGlobal()
onUnmounted(() => {
  unsubscribe()
})

const isActive = computed({
  get: () => Boolean(success.value || error.value),
  set: (value) => {
    if (!value) {
      success.value = null
      error.value = null
    }
  },
})

const color = computed(() => {
  if (success.value) {
    return 'success'
  }
  if (error.value) {
    return 'error'
  }
})
const message = computed(() => {
  if (success.value) {
    return success.value
  }
  if (error.value) {
    return error.value
  }
})
</script>
