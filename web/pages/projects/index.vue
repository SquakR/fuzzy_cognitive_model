<template>
  <VCard class="mt-2">
    <VCardTitle>{{ t('title') }}</VCardTitle>
    <VCardText>
      <VRow>
        <VCol cols="12">
          <ProjectAddForm @add-project="addProject">
            <template #activator="{ props }">
              <VBtn v-bind="props" color="primary">{{ t('addProject') }}</VBtn>
            </template>
          </ProjectAddForm>
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
          <UserAvatar :user="item.raw.creator" class="mr-1"></UserAvatar>
          {{ userNameFilter(item.raw.creator) }}</template
        >
        <template #item.createdAt="{ item }">
          {{ dateTimeFilter(item.raw.createdAt) }}
        </template>
        <template #item.updatedAt="{ item }">
          {{ dateTimeFilter(item.raw.updatedAt) }}
        </template>
        <template #item.actions="{ item }">
          <TableEdit class="mr-2" @click="editingProject = item.raw" />
          <TableDelete
            :item-name="t('project')"
            @confirm="deleteProject(item.raw.id)"
          />
        </template>
      </VDataTableServer>
      <ProjectChangeForm
        v-model="editingProject"
        @change-project="changeProject"
      />
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

const { t } = useI18n()

const headers = computed(() => [
  { key: 'name', title: t('name'), sortable: false },
  { key: 'description', title: t('description'), sortable: false },
  { key: 'creator', title: t('creator'), sortable: false },
  { key: 'createdAt', title: t('createdAt'), sortable: false },
  { key: 'updatedAt', title: t('updatedAt'), sortable: false },
  { key: 'actions', title: t('actions'), sortable: false },
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
  pending: projectsPending,
  refresh,
} = useGetProjects({ key: 'projects' }, projectsIn)
const {
  itemsLength,
  data: projects,
  insertAtTop,
  replace,
} = usePagination(
  projectsPagination,
  refresh,
  toRef(projectsIn.value, 'page'),
  toRef(projectsIn.value, 'perPage')
)
const addProject = async (project: ProjectOutType) => {
  await insertAtTop(project)
}
const changeProject = async (project: ProjectOutType) => {
  replace(project)
}

const {
  execute: deleteProject,
  pending: deleteProjectPending,
  onSuccess: deleteProjectOnSuccess,
} = useDeleteProject({ key: 'deleteProject' })
deleteProjectOnSuccess(() => {
  refresh()
})

const loading = computed(
  () => projectsPending.value || deleteProjectPending.value
)

const editingProject = ref<ProjectOutType | null>(null)
</script>

<i18n locale="en-US" lang="json">
{
  "title": "Projects",
  "addProject": "Add project",
  "name": "Name",
  "description": "Description",
  "creator": "Creator",
  "createdAt": "Date of creation",
  "updatedAt": "Date of change",
  "actions": "Actions",
  "project": "project"
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
  "updatedAt": "Дата изменения",
  "actions": "Действия",
  "project": "проект"
}
</i18n>
