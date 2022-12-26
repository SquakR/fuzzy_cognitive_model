<template>
  <div v-show="currentMessage" class="box the-global-message__box">
    <div class="notification" :class="notificationClasses">
      <button class="delete" @click="close" />
      {{ message }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { useGlobalMessagesStore } from '~~/store'

const { t } = useI18n()

const globalMessagesStore = useGlobalMessagesStore()

const currentMessageIndex = ref(0)
const currentMessage = computed(() =>
  currentMessageIndex.value < globalMessagesStore.messages.length
    ? globalMessagesStore.messages[currentMessageIndex.value]
    : null
)

const notificationClasses = computed(() => ({
  'is-success': currentMessage.value?.type === 'success',
  'is-danger': currentMessage.value?.type === 'fetchError',
}))

const message = computed(() => {
  if (!currentMessage.value) {
    return ''
  }
  if (currentMessage.value.type === 'success') {
    return t(currentMessage.value.message)
  }
  if (currentMessage.value.type === 'fetchError') {
    return currentMessage.value.message
  }
  return ''
})

const close = () => {
  if (currentMessage.value) {
    globalMessagesStore.deleteMessage(currentMessage.value.id)
  }
}
</script>

<style lang="sass">
.the-global-message__box
  position: fixed
  left: calc(50vw - 250px)
  bottom: 12vh
  width: 500px
  padding: 0
</style>
