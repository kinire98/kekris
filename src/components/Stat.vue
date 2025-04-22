<template>
  <div>
    <span>{{ props.statName }}</span> <span>{{ props.statValue }}</span>
  </div>
</template>
<style scoped>
div {
  position: relative;
  right: 5%;
  display: flex;
  align-items: center;
  justify-content: space-between;
  flex-direction: row;
  width: 110%;
  margin-block: 5px;
  padding: 10px;
  border-radius: 15px;
  background-color: #30303066;
  transition: all 0.3s;
  z-index: 99;
  padding-block: 20px;
  padding-inline: 15px;
}
div:hover {
  transform: scale(1.1);
  background-color: #303030;
}
</style>
<script setup lang="ts">
const props = defineProps<{
  statName: String;
  statValue: String;
}>();
</script>
