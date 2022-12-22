<template>
  <footer class="footer">
    <div class="content has-text-centered">
      <p>
        Copyright &#169; Orlov Artem <br />
        Version: {{ version }}
      </p>
    </div>
  </footer>
</template>

<script setup lang="ts">
const version = ref('0.1.0')
</script>
