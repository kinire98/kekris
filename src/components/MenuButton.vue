<template>
  <Button variant="outlined" raised @click="changeRoute" class="menu-button">
    <h2>{{ props.label }}</h2>
    <h3>{{ props.desc }}</h3>
  </Button>
</template>
<style scoped>
h2 {
  display: flex;
  align-items: start;
  justify-content: start;
  width: 100%;
  font-size: 25px;
  margin-top: 15px;
  margin-left: 25px;
}
h3 {
  display: flex;
  align-items: end;
  justify-content: end;
  width: 100%;
  font-size: 17px;
  margin-bottom: 15px;
  margin-right: 25px;
}
.menu-button {
  flex-direction: column;
  width: 45vw;
  height: 150px;
  border: 2.5px solid var(--main-color);
  color: var(--main-color);
  margin-block: 50px;
  font-size: 25px;
  transition: all 0.4s;
  display: flex;
  align-items: center;
  justify-content: start;
  cursor: pointer !important;
  /* padding-inline-start: 5vw; */
}
.menu-button:hover {
  width: 50vw;
  transform: translateX(-5vw) scale(1.1);
  border-color: var(--main-contrast) !important;
  color: var(--main-contrast) !important;
  background-color: var(--main-color) !important;
}
</style>
<script lang="ts" setup>
import Button from "primevue/button";
import { useRouter } from "vue-router";
const props = defineProps(["label", "path", "desc"]);
const router = useRouter();
function changeRoute() {
  router.push(props.path);
}
</script>
