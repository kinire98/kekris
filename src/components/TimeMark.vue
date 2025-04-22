<template>
  <div :class="classNameActive">
    <span
      >{{ position }}. <span v-if="props.position == 1" class="divider"></span
      >{{ mark }}</span
    >
  </div>
</template>
<style scoped>
div {
  text-align: justify;
  padding: 15px;
  font-size: 18px;
  border-radius: 15px;
  transition: all 0.3s;
  margin-block: 10px;
  background-color: #30303066;
}
div:hover {
  transform: scale(1.1);
  background-color: #303030;
}
.divider {
  width: 5px;
}
.current {
  background-color: var(--transparent-main-color) !important;
}
.current:hover {
  background-color: var(--main-color) !important;
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
