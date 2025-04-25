<template>
  <div id="wrapper">
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
        <h1 id="menu-title">{{ props.title }}</h1>
      </div>
      <Profile id="profile" />
    </div>
    <div id="holder">
      <div id="img-div">
        <a href="https://kinire98.github.io/kekris/" target="_blank">
          <img
            src="/logo-drop-shadow.png"
            :alt="$t('alts.logo')"
            class="logo"
          />
        </a>
      </div>
      <div id="buttons-div">
        <slot></slot>
      </div>
    </div>
<<<<<<< HEAD
<<<<<<< HEAD
=======
    <div v-if="props.exit" id="exit-icon">
      <Icon icon="radix-icons:exit" @click="exit" />
    </div>
>>>>>>> dev
=======
    <div v-if="props.exit" id="exit-icon">
      <Icon icon="radix-icons:exit" @click="exit" />
    </div>
>>>>>>> dev
  </div>
</template>
<style scoped>
#wrapper {
  display: flex;
  align-items: center;
  justify-content: space-around;
  flex-direction: column;
  z-index: 2;
}
img {
  transition: all 0.3s;
}
img:hover {
  transform: scale(1.1);
}
div#holder {
  display: flex;
  align-items: center;
  justify-content: center;
  flex-direction: row;
  height: 80vh;
}
#holder > #img-div {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 40vw;
  z-index: 999;
}
#holder > div > a {
  cursor: pointer;
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
div#buttons-div {
  display: flex;
  align-items: center;
  justify-content: center;
  flex-direction: column;
  width: 60vw;
  height: 100%;
}
#top-bar {
  width: 100vw;
  height: 15vh;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding-left: 5vw;
  padding-right: 5vw;
}

#menu-title {
  margin-left: 5vw;
}
#top-left-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 25vw !important;
}
<<<<<<< HEAD
<<<<<<< HEAD
</style>
<script setup lang="ts">
const props = defineProps({ title: String, back: Boolean });
=======
=======
>>>>>>> dev
#exit-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  position: absolute;
  width: 50px;
  height: 50px;
  left: 50px;
  bottom: 50px;
  * {
    width: 50px;
    height: 50px;
    color: var(--main-color);
    cursor: pointer;
    transition: all 0.3s;
  }
  *:hover {
    transform: scale(1.1);
  }
}
</style>
<script setup lang="ts">
import { Icon } from "@iconify/vue/dist/iconify.js";
import { invoke } from "@tauri-apps/api/core";
const props = defineProps({ title: String, back: Boolean, exit: Boolean });
<<<<<<< HEAD
>>>>>>> dev
=======
>>>>>>> dev
useI18n();
</script>
<script lang="ts">
import Button from "primevue/button";
import { useI18n } from "vue-i18n";
export default {
  methods: {
    goBack() {
      this.$router.back();
    },
<<<<<<< HEAD
<<<<<<< HEAD
=======
    exit() {
      invoke("exit_app");
    },
>>>>>>> dev
=======
    exit() {
      invoke("exit_app");
    },
>>>>>>> dev
  },
};
</script>
