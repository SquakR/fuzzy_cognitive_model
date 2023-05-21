<template>
  <div ref="container"></div>
</template>

<script setup lang="ts">
export interface Props {
  formula: string
}
const props = defineProps<Props>()

const container = ref<HTMLDivElement | null>(null)

const updateFormula = (formula: string) => {
  if (!container.value) {
    return
  }
  container.value.innerText = formula
  window.MathJax.typesetPromise([container.value])
}

watch(
  () => props.formula,
  (newValue) => {
    updateFormula(newValue)
  }
)
onMounted(() => {
  updateFormula(props.formula)
})
</script>
