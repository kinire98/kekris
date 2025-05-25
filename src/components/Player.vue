<template>
  <div>
    {{ props.name }}
    <span v-if="props.kick">Kick</span>
  </div>
</template>

<style scoped>
div {
  padding: 25px;
  width: 80%;
  border-radius: 25px;
  background-color: #30303066;
  font-size: 24px;
}
</style>
<script setup lang="ts">
const props = defineProps<{
  name: String;
  games_won: Number;
  games_played: Number;
  kick: Boolean;
}>();
</script>
