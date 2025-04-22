<template>
  <div @click="onImageClick">
    <img src="/defuser.png" :alt="$t('alts.user')" />
  </div>
</template>

<style scoped>
div {
  cursor: pointer;
  transition: all 0.3s;
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
import { useI18n } from "vue-i18n";

export default {
  methods: {
    onImageClick() {
      this.$router.push("/profile");
    },
  },
  setup() {
    const t = useI18n();
    return t;
  },
};
</script>
