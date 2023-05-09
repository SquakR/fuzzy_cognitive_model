<template>
  <VAvatar>
    <VImg
      v-if="user.avatar"
      :src="`${config.public.API_HTTP_BASE_URL}${user.avatar}`"
      :alt="user.lastName"
    ></VImg>
    <span v-else class="text-h6"
      >{{ user.lastName[0] }}{{ user.firstName[0] }}</span
    >
  </VAvatar>
</template>

<script setup lang="ts">
import { UserOutType } from '~/types'

export interface Props {
  user: UserOutType
}

defineProps<Props>()

const config = useRuntimeConfig()
</script>
