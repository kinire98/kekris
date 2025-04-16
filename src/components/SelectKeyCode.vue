<template>
  <div id="root">
    <span id="info">{{ props.info }}</span>
    <span id="value" @click="promptForChange">{{
      [
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
        " ",
      ].includes(keyValue!)
        ? $t("ui.controls.keys." + keyValue)
        : keyValue
    }}</span>
    <span id="description" v-if="props.desc != null" :title="props.desc">
      <Icon
        icon="iconamoon:question-mark-circle-duotone"
        class="icon-question-controls"
      />
    </span>
  </div>
  <Dialog
    v-model:visible="visible"
    modal
    :header="$t('ui.controls.change')"
    class="change-controls"
  >
    {{ $t("ui.controls.change-footer") }}
  </Dialog>
</template>
<style>
span.p-dialog-title {
  padding: 15px;
}
.desc-controls .p-dialog-content {
  max-width: 400px;
  text-wrap: pretty;
  text-align: justify;
}
.change-controls .p-dialog-content {
  width: 100%;
  text-align: center;
}
.icon-question-controls {
  color: var(--main-color);
  width: 25px;
  height: 50%;
  cursor: pointer;
  transition: all 0.3s;
}
.icon-question-controls:hover {
  transform: scale(1.2);
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
    text-align: center;
  }
}
#description {
  width: 5vw;
  height: 1.5vw;
  margin-left: 15px;
  * {
    width: 100%;
    height: 100%;
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
#info {
  margin-right: 15px;
}
</style>
<script setup lang="ts">
const props = defineProps({
  info: String,
  movementKey: String,
  value: String,
  desc: String,
});
const t = useI18n();
</script>
<script lang="ts">
import { Dialog } from "primevue";
import { Icon } from "@iconify/vue/dist/iconify.js";
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
      localStorage.setItem(this.movementKey!, e.key);
      document.removeEventListener("keydown", this.controlsChange);
    },
  },
};
</script>
