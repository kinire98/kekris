<template>
  <div id="holder-top">
    <div id="top-bar">
      <div id="top-left-bar">
        <Button
          label="<-"
          id="back"
          variant="outlined"
          raised
          @click="goBack"
          v-if="props.back == true"
        />
        <h1 id="menu-title" @click="showChangeUserNameDialog">
          {{ getUsername() }}
        </h1>
      </div>
    </div>
    <div>
      <slot></slot>
    </div>
  </div>
  <Dialog v-model:visible="visible" modal header="Select username">
    <InputText
      type="text"
      v-model="userNameValue"
      :invalid="userNameValue.length === 0"
    />
    <br />
    <Button
      label="Confirm"
      variant="outlined"
      raised
      @click="onDialogClick"
      id="dialog-button"
    ></Button>
  </Dialog>
</template>
<style scoped>
h1 {
  color: var(--white-contrast);
  cursor: pointer;
  transition: all 0.1s;
}
h1:hover {
  transform: scale(1.1);
}
#holder-top {
  display: flex;
  align-items: center;
  justify-content: center;
  flex-direction: column;
}
#top-bar {
  width: 100vw;
  height: 15vh;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding-left: 5vw;
  padding-right: 15vw;
}

#menu-title {
  margin-left: 5vw;
}
#top-left-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
}
#back {
  width: 5vw;
  height: 50px;
  border-color: var(--main-color);
  color: var(--main-color);
  margin-block: 50px;
  font-size: 25px;
  transition: all 0.3s;
  display: flex;
  align-items: center;
  justify-content: center;
}
#back:hover {
  border-color: var(--main-contrast) !important;
  color: var(--main-contrast) !important;
  background-color: var(--main-color) !important;
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
const props = defineProps({ title: String, back: Boolean });
</script>
<script lang="ts">
import Button from "primevue/button";

import Dialog from "primevue/dialog";

import InputText from "primevue/inputtext";
import { getUsername, setUsername } from "../helpers/username";
export default {
  data() {
    return { visible: false, userNameValue: getUsername()! };
  },
  methods: {
    goBack() {
      this.$router.back();
    },
    showChangeUserNameDialog() {
      this.visible = true;
      console.log(this.userNameValue);
    },
    onDialogClick() {
      console.log("here");
      if (this.userNameValue != null && this.userNameValue.length != 0) {
        this.visible = false;
        setUsername(this.userNameValue);
      }
    },
  },
};
</script>
