<template>
  <div @click="">
    <img src="/logo-drop-shadow.png" alt="" />
    <h2>Click anywhere</h2>
  </div>
  <Dialog
    v-model:visible="visible"
    modal
    header="Select username"
    closable="false"
    @update:visible="onDialogChange"
  >
    <InputText type="text" v-model="value" />
    <Button
      label="Confirm"
      variant="outlined"
      raised
      @click="visible = false"
    ></Button>
  </Dialog>
</template>
<style scoped>
div {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 85vh;
  width: 100vw;
  flex-direction: column;
  * {
    margin-block: 5rem;
  }
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
button {
  border-color: var(--main-color);
  color: var(--main-color);
  margin-block: 50px;
  transition: all 0.3s;
  display: flex;
  align-items: center;
  justify-content: start;
}
button:hover {
  border-color: var(--main-contrast) !important;
  color: var(--main-contrast) !important;
  background-color: var(--main-color) !important;
}
</style>

<script lang="ts">
import Dialog from "primevue/dialog";

import Button from "primevue/button";
import InputText from "primevue/inputtext";
import { getUsername, setUsername } from "../helpers/username";

export default {
  data() {
    return { visible: false, value: "" };
  },
  methods: {
    changeRouteChecking() {
      let username = getUsername();
      if (username == null) {
        this.visible = true;
      } else {
        this.$router.push("/main");
      }
    },
    onDialogChange(visible: boolean) {
      if (visible) {
        return;
      }
      setUsername(this.value);
      this.$router.push("/main");
    },
  },
  mounted() {
    document.body.addEventListener("click", this.changeRouteChecking);
  },
  beforeDestroy() {
    document.body.removeEventListener("click", this.changeRouteChecking);
  },
};
</script>
