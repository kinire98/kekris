<template>
  <MenuBackLayout :title="$t('ui.appeareance.appeareance')" back>
    <div id="themes">
      <Theme
        v-for="(theme, name) in themes"
        :name="capitalizeFirstLetter(name)"
        :theme="theme"
        :active="themeName == name"
        @click="changeTheme(name)"
      />
    </div>
  </MenuBackLayout>
</template>
<style scoped>
div#themes {
  overflow-y: scroll;
  overflow-x: hidden;
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: start;
  flex-direction: column;
}
</style>
<script setup lang="ts">
import { useI18n } from "vue-i18n";
import MenuBackLayout from "../layouts/MenuBackLayout.vue";
import Theme from "../components/Theme.vue";
import { themes } from "../board/colors";
import { getTheme, setTheme } from "../helpers/themes";
import { ref } from "vue";

const t = useI18n();
let themeName = ref(getTheme() ?? "neon");
function capitalizeFirstLetter(stringVal: string): string {
  return stringVal.charAt(0).toUpperCase() + stringVal.slice(1);
}
function changeTheme(name: string) {
  themeName.value = name;
  setTheme(name);
}
</script>
