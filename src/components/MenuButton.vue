<template>
  <div>
    <Button variant="outlined" raised @click="changeRoute" class="menu-button">
      <h2>{{ props.label }}</h2>
      <h3>{{ props.desc }}</h3>
    </Button>
  </div>
</template>
<style scoped>
div {
  position: relative;
  display: flex;
  align-items: end;
  justify-content: end;
  width: 100%;
}
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
  width: 80%;
  height: 150px;
  border-block: 2.5px solid var(--main-color);
  border-left: 2.5px solid var(--main-color);
  border-right: none;
  color: var(--main-color);
  margin-block: 50px;
  font-size: 25px;
  transition: all 0.3s;
  display: flex;
  align-items: center;
  justify-content: start;
  cursor: pointer !important;
  /* padding-inline-start: 5vw; */
}
.menu-button:hover {
  width: 50vw;
  transform: translateX(calc(75% * 0.05 * -1)) scale(1.1);
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
