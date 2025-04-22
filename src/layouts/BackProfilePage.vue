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
          <span>{{ getUsername() }}</span
          ><Icon icon="mdi-light:pencil" />
        </h1>
      </div>
    </div>
    <div id="content">
      <slot></slot>
    </div>
  </div>
  <Dialog v-model:visible="visible" modal :header="$t('ui.username.select')">
    <InputText
      type="text"
      v-model="userNameValue"
      :invalid="userNameValue.length === 0"
    />
    <br />
    <Button
      :label="$t('ui.username.confirm')"
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
  display: flex;
  align-items: center;
  justify-content: space-between;
  span {
    transition: all 0.3s;
    margin-right: 20px;
  }
}
h1:hover span {
  transform: scale(1.1);
}

#holder-top {
  display: flex;
  align-items: center;
  justify-content: center;
  flex-direction: column;
  height: 100vh;
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
#content {
  width: 100%;
  height: 85vh;
}
</style>
<script setup lang="ts">
const props = defineProps({ title: String, back: Boolean });
useI18n();
</script>
<script lang="ts">
import Button from "primevue/button";

import Dialog from "primevue/dialog";

import InputText from "primevue/inputtext";
import { getUsername, setUsername } from "../helpers/username";
import { useI18n } from "vue-i18n";
import { Icon } from "@iconify/vue/dist/iconify.js";
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
    },
    onDialogClick() {
      if (this.userNameValue != null && this.userNameValue.length != 0) {
        this.visible = false;
        setUsername(this.userNameValue);
      }
    },
  },
};
</script>
