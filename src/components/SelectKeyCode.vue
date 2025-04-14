<template>
  <div>
    <span id="info">{{ props.info }}</span>
    <span id="value" @click="promptForChange">{{
      keyValue == " "
        ? $t("ui.controls.keys.space")
        : [
            "ArrowRight",
            "ArrowLeft",
            "ArrowDown",
            "ArrowUp",
            "Delete",
            "Backspace",
            "Home",
            "End",
            "PageUp",
            "PageDown",
            "Shift",
            "CapsLock",
          ].includes(keyValue!)
        ? $t("ui.controls.keys." + keyValue)
        : keyValue
    }}</span>
  </div>
  <Dialog v-model:visible="visible" modal :header="$t('ui.controls.change')">
    {{ $t("ui.controls.change-footer") }}
  </Dialog>
</template>
<style>
span.p-dialog-title {
  padding: 15px;
}
.p-dialog-content {
  text-align: center;
}
</style>
<style scoped>
div {
  display: flex;
  align-items: center;
  justify-content: space-evenly;
  width: 85%;
  background-color: #30303066;
  padding: 15px;
  border-radius: 15px;
  margin-block: 15px;
  * {
    width: 85%;
    display: flex;
    align-items: center;
    justify-content: center;
  }
}
#value {
  background-color: #30303066;
  padding: 10px;
  border-radius: 15px;
  cursor: pointer;
  transition: all 0.3s;
}
#value:hover {
  transform: scale(1.05);
}
.p-button-outlined {
  border: 2.5px solid var(--main-color) !important;
  color: var(--main-color) !important;
  margin-block: 10px;
  transition: all 0.3s;
  display: flex;
  align-items: center;
  justify-content: start;
}
.p-button-outlined:hover {
  border-color: var(--main-contrast) !important;
  color: var(--main-contrast) !important;
  background-color: var(--main-color) !important;
}
.p-inputtext {
  border: 1px solid var(--transparent-main-color);
  margin-bottom: 5px;
}
.p-inputtext:enabled:focus {
  border: 1px solid var(--main-color);
}
</style>
<script setup lang="ts">
const props = defineProps({
  info: String,
  movementKey: String,
  value: String,
});
const t = useI18n();
</script>
<script lang="ts">
import { Dialog } from "primevue";
import { InputText } from "primevue";
import { Button } from "primevue";
import { useI18n } from "vue-i18n";
export default {
  data() {
    return {
      visible: false,
      keyValue: this.value,
    };
  },
  methods: {
    promptForChange() {
      this.visible = true;
      document.addEventListener("keydown", this.controlsChange);
    },
    controlsChange(e: KeyboardEvent) {
      this.visible = false;
      this.keyValue = e.key;
      document.removeEventListener("keydown", this.controlsChange);
    },
  },
};
</script>
