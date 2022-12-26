<template>
  <div v-show="currentFetchResult" class="box the-global-message__box">
    <div class="notification" :class="notificationClasses">
      <button class="delete" @click="close" />
      {{ message }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { useFetchResultStore } from '~~/store'

const { t } = useI18n()

const fetchResultStore = useFetchResultStore()

const currentFetchResultIndex = ref(0)
const currentFetchResult = computed(() =>
  currentFetchResultIndex.value < fetchResultStore.activeFetchResults.length
    ? fetchResultStore.activeFetchResults[currentFetchResultIndex.value]
    : null
)

const notificationClasses = computed(() => ({
  'is-success': currentFetchResult.value?.result.success,
  'is-danger': currentFetchResult.value?.result.fetchError,
}))

const message = computed(() => {
  if (!currentFetchResult.value) {
    return ''
  }
  if (currentFetchResult.value.result.success) {
    return t(currentFetchResult.value.result.success)
  }
  if (currentFetchResult.value.result.fetchError) {
    return currentFetchResult.value.result.fetchError.message
  }
  return ''
})

const close = () => {
  if (currentFetchResult.value) {
    currentFetchResult.value.clearResult()
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
