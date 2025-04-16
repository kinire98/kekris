<template>
  <div :class="classActive">
    <h2>{{ props.name }}</h2>
    <canvas :id="canvasId" width="750" height="150"></canvas>
  </div>
</template>
<style scoped>
h2 {
  width: 100%;
  text-align: left;
}
div {
  display: flex;
  align-items: center;
  justify-content: center;
  flex-direction: column;
  transition: all 0.3s;
  width: 75%;
  min-width: 775px;
  cursor: pointer;
  background-color: #30303066;
  border-radius: 15px;
  margin-block: 15px;
  padding-inline: 25px;
  padding-bottom: 25px;
}
div:hover {
  transform: scale(1.05);
}
canvas {
  background-color: #30303066;
  border-radius: 15px;
}
.active {
  background-color: var(--transparent-main-color);
}
</style>
<script setup lang="ts">
import { computed, onMounted } from "vue";
import { Theme } from "../types/Theme";
import drawThemeTest from "../helpers/drawThemeTest";

const props = defineProps<{
  name: String;
  theme: Theme;
  active: Boolean;
}>();
const canvasId = uncapitalizeFirstLetter(props.name.toString());
function uncapitalizeFirstLetter(stringVal: string): string {
  return stringVal.charAt(0).toLowerCase() + stringVal.slice(1);
}
onMounted(() => {
  const $canvas = document.getElementById(canvasId)! as HTMLCanvasElement;
  drawThemeTest($canvas, props.theme);
});

const classActive = computed(() => (props.active ? "active" : "not-active"));
</script>
