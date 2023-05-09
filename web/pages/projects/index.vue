<template>
  <VCard class="mt-2">
    <VCardTitle>{{ t('title') }}</VCardTitle>
    <VCardText>
      <VRow>
        <VCol cols="12">
          <AddProjectForm @add-project="addProject">
            <template #activator="{ props }">
              <VBtn v-bind="props" color="primary">{{ t('addProject') }}</VBtn>
            </template>
          </AddProjectForm>
        </VCol>
      </VRow>
      <VDataTableServer
        v-model:items-per-page="projectsIn.perPage"
        v-model:page="projectsIn.page"
        :headers="headers"
        :loading="loading"
        :items-length="itemsLength"
        :items="projects"
      >
        <template #item.creator="{ item }">
          <BaseUserAvatar
            :user="item.raw.creator"
            class="mr-1"
          ></BaseUserAvatar>
          {{ userNameFilter(item.raw.creator) }}</template
        >
        <template #item.createdAt="{ item }">
          {{ dateTimeFilter(item.raw.createdAt) }}
        </template>
        <template #item.updatedAt="{ item }">
          {{ dateTimeFilter(item.raw.updatedAt) }}
        </template>
      </VDataTableServer>
    </VCardText>
  </VCard>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { VDataTableServer } from 'vuetify/labs/VDataTable'
import { ProjectOutType, ProjectsInType } from '~/types'
import { dateTimeFilter, userNameFilter } from '~/utils'

definePageMeta({
  middleware: 'auth',
})

type LocalProjectsInType = ProjectsInType & {
  page: number
  perPage: number
}

const { t } = useI18n({})

const headers = computed(() => [
  { key: 'name', title: t('name'), sortable: false },
  { key: 'description', title: t('description'), sortable: false },
  { key: 'creator', title: t('creator'), sortable: false },
  { key: 'createdAt', title: t('createdAt'), sortable: false },
  { key: 'updatedAt', title: t('updatedAt'), sortable: false },
])
const projectsIn = ref<LocalProjectsInType>({
  group: 'private',
  statuses: ['creator', 'member'],
  search: null,
  isArchived: null,
  createdAtStart: null,
  createdAtIncludeStart: null,
  createdAtEnd: null,
  createdAtIncludeEnd: null,
  updatedAtStart: null,
  updatedAtIncludeStart: null,
  updatedAtEnd: null,
  updatedAtIncludeEnd: null,
  page: 1,
  perPage: 10,
})
const {
  data: projectsPagination,
  pending: loading,
  refresh,
} = useGetProjects({ key: 'projects' }, projectsIn)
const {
  itemsLength,
  data: projects,
  insertAtTop,
} = usePagination(
  projectsPagination,
  refresh,
  toRef(projectsIn.value, 'page'),
  toRef(projectsIn.value, 'perPage')
)
const addProject = async (project: ProjectOutType) => {
  await insertAtTop(project)
}
</script>

<i18n locale="en-US" lang="json">
{
  "title": "Projects",
  "addProject": "Add project",
  "name": "Name",
  "description": "Description",
  "creator": "Creator",
  "createdAt": "Date of creation",
  "updatedAt": "Date of change"
}
</i18n>

<i18n locale="ru-RU" lang="json">
{
  "title": "Проекты",
  "addProject": "Добавить проект",
  "name": "Название",
  "description": "Описание",
  "creator": "Создатель",
  "createdAt": "Дата создание",
  "updatedAt": "Дата изменения"
}
</i18n>
