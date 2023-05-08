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
const isActive = computed({
  get: () => messageStore.target === 'global' && !!messageStore.message,
  set: (value) => {
    if (!value) {
      messageStore.message = null
    }
  },
})

const color = computed(() => {
  if (!messageStore.message) {
    return undefined
  }
  return messageStore.message.type
})
const message = computed(() => {
  if (!messageStore.message) {
    return undefined
  }
  return messageStore.message.message
})
</script>
