<template>
  <div>{{ props.statName }} : {{ props.statValue }}</div>
</template>
<style scoped>
div {
  display: flex;
  align-items: center;
  justify-content: center;
}
</style>
<script setup lang="ts">
const props = defineProps<{
  statName: String;
  statValue: String;
}>();
</script>
