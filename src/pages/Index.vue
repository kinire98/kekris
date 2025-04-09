<template>
  <div @click="">
    <img src="/logo-drop-shadow.png" alt="" />
    <h2>Click anywhere</h2>
  </div>
  <div id="front"></div>
  <Dialog v-model:visible="visible" modal header="Select username">
    <InputText type="text" v-model="value" :invalid="!value" />
    <Button
      label="Confirm"
      variant="outlined"
      raised
      @click="onConfirmed"
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
.p-inputtext {
  border: 1px solid var(--main-color);
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
};
</script>
