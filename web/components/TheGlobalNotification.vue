<template>
  <div
    v-show="globalNotificationStore.notification"
    class="box the-global-message__box"
  >
    <div class="notification" :class="notificationClasses">
      <button class="delete" @click="close" />
      {{ globalNotificationStore.message }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { useGlobalNotificationStore } from '~~/store'

const globalNotificationStore = useGlobalNotificationStore()

const notificationClasses = computed(() => ({
  'is-success': globalNotificationStore.notification?.type === 'success',
  'is-danger': globalNotificationStore.notification?.type === 'fetchError',
}))

const close = () => {
  globalNotificationStore.deleteNotification()
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
