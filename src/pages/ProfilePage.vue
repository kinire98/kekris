<script setup lang="ts">
import { getUsername } from "../helpers/username";
import BackProfilePage from "../layouts/BackProfilePage.vue";
const username = getUsername()!;
</script>

<template>
  <BackProfilePage :title="username" back> </BackProfilePage>
</template>
