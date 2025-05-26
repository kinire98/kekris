<template>
  <div>
    <div id="buttons">
      <Button id="back" variant="outlined" raised @click="mainMenu">
        <h2>{{ $t("ui.again.main_menu") }}</h2>
      </Button>
      <Button id="back" variant="outlined" raised @click="tryAgain">
        <h2>{{ $t("ui.again.try_again") }}</h2>
      </Button>
    </div>
  </div>
</template>
<style scoped>
div:not(#buttons) {
  display: flex;
  align-items: center;
  justify-content: space-evenly;
  flex-direction: column;
  height: 100vh;
}
#buttons {
  display: flex;
  align-items: center;
  justify-content: space-evenly;
  flex-direction: row;
  width: 50vw;
}
.p-button-outlined {
  border: 2.5px solid var(--main-color) !important;
  color: var(--main-color) !important;
  margin-block: 10px;
  transition: all 0.3s;
  display: flex;
  align-items: center;
  justify-content: start;
  cursor: pointer;
  font-size: 15px;
  margin-inline: 25px;
  padding: 15px;
  padding-inline: 50px;
}
.p-button-outlined:hover {
  border-color: var(--main-contrast) !important;
  color: var(--main-contrast) !important;
  background-color: var(--main-color) !important;
  transform: scale(1.1);
}
</style>
<script setup lang="ts">
import { useI18n } from "vue-i18n";
import { router } from "../router";
useI18n();

function mainMenu() {
  router.push("/main");
}
function tryAgain() {
  router.back();
}
</script>
