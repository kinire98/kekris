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
        <h1 id="menu-title">{{ props.title }}</h1>
      </div>
    </div>
    <div>
      <slot></slot>
    </div>
  </div>
</template>
<style scoped>
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
</style>
<script setup lang="ts">
const props = defineProps({ title: String, back: Boolean });
</script>
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
