<template>
  <div @click="" id="logo">
    <img src="/logo-drop-shadow.png" alt="" />
    <h2>{{ $t("ui.index.click") }}</h2>
  </div>
  <div id="front"></div>
  <Dialog v-model:visible="visible" modal :header="$t('ui.username.select')">
    <InputText type="text" v-model="value" :invalid="!value" />
    <Button
      :label="$t('ui.username.confirm')"
      variant="outlined"
      raised
      @click="onConfirmed"
    ></Button>
  </Dialog>
</template>
<style scoped>
#logo {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 95vh;
  width: 100vw;
  flex-direction: column;
  * {
    margin-block: 5rem;
  }
}
#front {
  position: absolute;
  top: 0;
  bottom: 0;
  left: 0;
  right: 0;
  z-index: 2;
}
h2 {
  animation: opa 2s ease-in infinite;
}
@keyframes opa {
  0% {
    opacity: 1;
  }
  50% {
    opacity: 0;
  }
  100% {
    opacity: 1;
  }
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

<script lang="ts">
import Dialog from "primevue/dialog";

import Button from "primevue/button";
import InputText from "primevue/inputtext";
import { getUsername, setUsername } from "../helpers/username";
import { useI18n } from "vue-i18n";

export default {
  data() {
    return { visible: false, value: "" };
  },
  methods: {
    changeRouteChecking() {
      let username = getUsername();
      if (username == null || username.length == 0) {
        this.visible = true;
      } else {
        this.$router.push("/main");
      }
    },
    onConfirmed() {
      if (this.value.length > 0) {
        setUsername(this.value);
        this.$router.push("/main");
      }
    },
  },
  mounted() {
    document
      .getElementById("front")!
      .addEventListener("click", this.changeRouteChecking);
  },
  setup() {
    const t = useI18n();
    return { t };
  },
};
</script>
