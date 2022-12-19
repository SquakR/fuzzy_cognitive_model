<template>
  <div>
    <div>{{ v }}</div>
    <input type="button" value="plus" @click="plus" />
  </div>
</template>

<script setup lang="ts">
const v = ref(0)
const plus = () => {
  v.value += 10
}
</script>
