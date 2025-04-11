<template>
  <MenuLayout>
    <Toast class="not-yet-implemented" position="bottom-right" />
    <div>
      <MenuButton label="Single player" path="/singleplayer" />
      <MenuButton label="Multiplayer" path="/multiplayer" />
      <MenuButton label="Settings" path="/settings" />
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
</style>
<script lang="ts">
import MenuLayout from "../layouts/MenuLayout.vue";
import MenuButton from "../components/MenuButton.vue";
import { Toast } from "primevue";
import { useToast } from "primevue";
export default {
  beforeRouteLeave(to, _from, next) {
    if (to.path === "/multiplayer") {
      const toast = useToast();
      toast.add({
        severity: "contrast",
        summary: "Multiplayer soon available",
        life: 2500,
      });
      next(false);
    }
  },
};
</script>
