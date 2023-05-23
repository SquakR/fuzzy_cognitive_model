<template>
  <template v-if="selectedConnection">
    <VTabs v-model="tab" bg-color="teal-lighten-1">
      <VTab value="connection">{{ t('connection') }}</VTab>
      <VTab v-if="plugins.adjustment.isInstalled" value="adjustment">{{
        t('adjustment')
      }}</VTab>
    </VTabs>
    <VCardText ref="content" class="drawer-card-text">
      <VWindow v-model="tab">
        <VWindowItem value="connection">
          <ModelChangeConnectionForm
            :model="model"
            :cy="cy"
            :selected-connection="selectedConnection"
            :concept-constraints-plugin="plugins.conceptConstraints"
            :change-connection="changeConnection"
            :delete-connection="deleteConnection"
            :delete-connection-pending="deleteConnectionPending"
          />
        </VWindowItem>
        <VWindowItem v-if="plugins.adjustment.isInstalled" value="adjustment">
          <PluginsControlConnectionsChangeConnectionForm
            :selected-connection="selectedConnection"
            :control-connections-plugin="plugins.controlConnections"
          />
          <VDivider />
          <PluginsConnectionConstraintsChangeConnectionForm
            :selected-connection="selectedConnection"
            :connection-constraints-plugin="plugins.connectionConstraints"
          />
          <VDivider />
        </VWindowItem>
      </VWindow>
    </VCardText>
  </template>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { ConnectionOutType, ModelOutType, Plugins } from '~/types'

export interface Props {
  model: ModelOutType
  plugins: Plugins
  selectedConnection: ConnectionOutType | null
  cy: cytoscape.Core
  changeConnection: ReturnType<typeof useModelActions>['changeConnection']
  deleteConnection: ReturnType<typeof useModelActions>['deleteConnection']
  deleteConnectionPending: boolean
}
defineProps<Props>()

const { t } = useI18n()

const tab = ref<'connection' | 'adjustment' | null>(null)
</script>

<i18n locale="en-US" lang="json">
{
  "connection": "Connection",
  "adjustment": "Adjustment"
}
</i18n>

<i18n locale="ru-RU" lang="json">
{
  "connection": "Связь",
  "adjustment": "Настройка"
}
</i18n>
