<template>
  <div :class="classNameActive">{{ position }}. {{ mark }}</div>
</template>
<style scoped>
div {
  display: flex;
  align-items: center;
  justify-content: center;
}
</style>
<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{
  position: Number;
  mark: String;
  current: Boolean;
}>();
let classNameActive = computed(() => (props.current ? "current" : ""));
</script>
