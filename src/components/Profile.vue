<template>
  <div @click="onImageClick">
    <img src="/defuser.png" alt="User image" />
  </div>
</template>

<style scoped>
div {
  cursor: pointer;
  transition: all 0.3s;
  margin-left: 55vw;
  z-index: 9999;
}
div:hover {
  transform: scale(1.1);
}
img {
  width: 100px;
  height: 100px;
  border-radius: 100%;
}
</style>

<script>
export default {
  methods: {
    onImageClick() {
      this.$router.push("/profile");
    },
  },
};
</script>
