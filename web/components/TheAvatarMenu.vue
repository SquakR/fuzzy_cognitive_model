<template>
  <VMenu>
    <template #activator="{ props }">
      <VBtn size="large" v-bind="props" icon>
        <VAvatar size="large" color="white" class="text-indigo-darken-1">
          <VImg
            v-if="user.avatar"
            :src="`${config.public.API_HTTP_BASE_URL}${user.avatar}`"
            :alt="user.lastName"
          ></VImg>
          <span v-else class="text-h6"
            >{{ user.lastName[0] }}{{ user.firstName[0] }}</span
          >
        </VAvatar>
      </VBtn>
    </template>
    <VSheet>
      <VList density="compact">
        <VListItem prepend-icon="mdi-vector-polygon-variant">
          <VListItemTitle>{{ t('projects') }}</VListItemTitle>
        </VListItem>
        <VListItem prepend-icon="mdi-logout" @click="signOut">
          <VListItemTitle>{{ t('signOut') }}</VListItemTitle>
        </VListItem>
      </VList>
    </VSheet>
  </VMenu>
</template>

<script setup lang="ts">
import { UserOutType } from '~/types'
import { useI18n } from 'vue-i18n'
import { useUserStore } from '~/store'

export interface Props {
  user: UserOutType
}

defineProps<Props>()

const config = useRuntimeConfig()

const { t } = useI18n({})

const userStore = useUserStore()

const signOut = useSignOut({
  key: 'signOut',
  onSuccess: () => {
    userStore.user = null
  },
})
</script>

<i18n locale="en-US" lang="json">
{
  "projects": "Projects",
  "signOut": "Sign out"
}
</i18n>

<i18n locale="ru-RU" lang="json">
{
  "projects": "Проекты",
  "signOut": "Выйти"
}
</i18n>
