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
  get: () => Boolean(messageStore.globalSuccess || messageStore.globalError),
  set: (value) => {
    if (!value) {
      messageStore.globalSuccess = null
      messageStore.globalError = null
    }
  },
})

const color = computed(() => {
  if (messageStore.globalSuccess) {
    return 'success'
  }
  if (messageStore.globalError) {
    return 'error'
  }
})
const message = computed(() => {
  if (messageStore.globalSuccess) {
    return messageStore.globalSuccess
  }
  if (messageStore.globalError) {
    return messageStore.globalError
  }
})
</script>
