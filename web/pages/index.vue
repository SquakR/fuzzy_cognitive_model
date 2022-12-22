<template>
  <form class="mt-2" style="width: 500px" @submit.prevent="fetchUser()">
    <div class="card">
      <header class="card-header">
        <p class="card-header-title">User View</p>
      </header>
      <div class="card-content">
        <BaseTextField
          v-model="userId"
          label="User id"
          placeholder="Enter user id"
        ></BaseTextField>
        <pre v-if="user">User: {{ user }}</pre>
        <div v-else>User not loaded</div>
      </div>
      <footer class="card-footer">
        <div class="card-footer-item is-flex is-justify-content-flex-end">
          <BaseButton type="submit" :loading="userLoading"
            >Fetch user</BaseButton
          >
        </div>
      </footer>
    </div>
  </form>
</template>

<script setup lang="ts">
const userId = ref('')
const {
  pending: userLoading,
  data: user,
  execute: fetchUser,
} = useAPIUser(computed(() => Number.parseInt(userId.value)))
</script>
