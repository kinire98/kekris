<template>
  <Button variant="outlined" raised @click="changeRoute" class="menu-button">
    {{ props.label }}
  </Button>
</template>
<style scoped>
.menu-button {
  width: 45vw;
  height: 150px;
  border-color: var(--main-color);
  color: var(--main-color);
  margin-block: 50px;
  font-size: 25px;
  transition: all 0.3s;
  display: flex;
  align-items: center;
  justify-content: start;
  padding-inline-start: 5vw;
}
.menu-button:hover {
  width: 50vw;
  border-color: var(--main-contrast) !important;
  color: var(--main-contrast) !important;
  background-color: var(--main-color) !important;
}
</style>
<script lang="ts" setup>
import Button from "primevue/button";
import { useRouter } from "vue-router";
const props = defineProps(["label", "path"]);
const router = useRouter();
function changeRoute() {
  router.push(props.path);
}
</script>
