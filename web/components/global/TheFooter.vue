<template>
  <footer
    class="flex h-20 w-full flex-col items-center justify-center bg-emerald-500 text-orange-50"
  >
    <div>Copyright &#169; Orlov Artem</div>
    <div>Version: {{ version }}</div>
  </footer>
</template>

<script setup lang="ts">
const version = ref('0.1.0')
</script>
