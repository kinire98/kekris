<template>
  <div id="holder">
    <div>
      <a href="https://kinire98.github.io/kekris/">
        <img src="/logo-drop-shadow.png" alt="" class="logo" />
      </a>
    </div>
    <slot></slot>
  </div>
</template>
<style scoped>
div#holder {
  display: flex;
  align-items: center;
  justify-content: center;
  flex-direction: row;
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
</style>
<script lang="ts">
export default {};
</script>
