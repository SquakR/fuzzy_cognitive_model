<template>
  <BaseForm
    :action-key="SET_IS_CONTROL_CONNECTION_KEY"
    :title="t('isControl')"
    :button-text="t('buttonText')"
    :validation-schema="validationSchema"
    :initial-values="initialValues"
    :on-submit="onSubmit"
    width="468"
    flat
  >
    <BaseCheckbox :label="t('isControl')" name="isControl" />
  </BaseForm>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import * as yup from 'yup'
import {
  ConnectionOutType,
  ControlConnectionsPlugin,
  SET_IS_CONTROL_CONNECTION_KEY,
} from '~/types'

export interface Props {
  selectedConnection: ConnectionOutType
  controlConnectionsPlugin: ControlConnectionsPlugin
}

const props = defineProps<Props>()

const { $yup } = useNuxtApp()
const { t } = useI18n()

const validationSchema = $yup.object({
  isControl: $yup.boolean().required(),
})
const initialValues = computed(() => ({
  isControl: props.selectedConnection.pluginsData.controlConnections!.isControl,
}))
const onSubmit = async (values: yup.InferType<typeof validationSchema>) => {
  await props.controlConnectionsPlugin.setIsControl(
    props.selectedConnection.id,
    values.isControl
  )
}
</script>

<i18n locale="en-US" lang="json">
{
  "isControl": "Control Connection",
  "buttonText": "Change"
}
</i18n>

<i18n locale="ru-RU" lang="json">
{
  "isControl": "Управляющая связь",
  "buttonText": "Изменить"
}
</i18n>
