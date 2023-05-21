<template>
  <Teleport to="#change-connection-drawer">
    <VExpandXTransition>
      <VCard
        v-show="isActive"
        class="drawer-card"
        :width="tab === 'adjustment' ? 520 : 500"
      >
        <VTabs v-model="tab" bg-color="teal-lighten-1">
          <VTab value="connection">{{ t('connection') }}</VTab>
          <VTab v-if="plugins.adjustment.isInstalled" value="adjustment">{{
            t('adjustment')
          }}</VTab>
        </VTabs>
        <VCardText v-if="selectedConnection" class="drawer-card-text">
          <VWindow v-model="tab">
            <VWindowItem value="connection"></VWindowItem>
            <VWindowItem
              v-if="plugins.adjustment.isInstalled"
              value="adjustment"
            />
          </VWindow>
        </VCardText>
      </VCard>
    </VExpandXTransition>
  </Teleport>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { ConnectionOutType, ModelOutType, Plugins } from '~/types'

export interface Props {
  model: ModelOutType
  plugins: Plugins
  cy: cytoscape.Core
}
const props = defineProps<Props>()

const { t } = useI18n()

const selectedConnection = ref<ConnectionOutType | null>(null)
const isActive = computed(() => Boolean(selectedConnection.value))

const tab = ref<'connection' | 'adjustment' | null>(null)

props.cy.on('select', 'edge', (e) => {
  selectedConnection.value = props.model.connections.find(
    (connection) => connection.id === e.target.data().connectionId
  )!
})
props.cy.on('unselect', 'edge', () => {
  selectedConnection.value = null
})
props.cy.on('remove', 'edge', () => {
  selectedConnection.value = null
})
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
