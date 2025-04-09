<template>
  <MenuLayout>
    <div>
      <Button label="Single player" variant="outlined" raised></Button>
      <Button label="Multiplayer" variant="outlined" raised></Button>
      <Button label="Settings" variant="outlined" raised></Button>
    </div>
  </MenuLayout>
</template>
<style scoped>
div {
  display: flex;
  align-items: end;
  justify-content: center;
  flex-direction: column;
  height: 100vh;
  width: 100vw;
}
button {
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
button:hover {
  width: 50vw;
  border-color: var(--main-contrast) !important;
  color: var(--main-contrast) !important;
  background-color: var(--main-color) !important;
}
</style>
<script lang="ts">
import Button from "primevue/button";
import MenuLayout from "../layouts/MenuLayout.vue";
export default {};
</script>
