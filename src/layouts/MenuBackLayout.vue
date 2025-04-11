<template>
  <div id="holder">
    <div>
      <a href="https://kinire98.github.io/kekris/">
        <img src="/logo-drop-shadow.png" alt="" class="logo" />
      </a>
    </div>
    <slot></slot>
  </div>
  <Button label="<-" id="back" variant="outlined" raised @click="goBack" />
</template>
<style scoped>
div#holder {
  display: flex;
  align-items: center;
  justify-content: center;
  flex-direction: row;
  height: 100vh;
}
#holder > div {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 100%;
}
#holder > div > a {
  cursor: pointer;
}
#back {
  position: absolute;
  top: 2vh;
  left: 3vw;
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
</style>
<script lang="ts">
import Button from "primevue/button";
export default {
  methods: {
    goBack() {
      this.$router.back();
    },
  },
};
</script>
