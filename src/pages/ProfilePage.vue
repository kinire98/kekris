<template>
  <BackProfilePage back>
    <div id="holder-stats-profile">
      <ProfileStatsPanel
        :title="$t('ui.profile.classic')"
        :command-name="classicCommand"
      />
      <ProfileStatsPanel
        :title="$t('ui.profile.lines')"
        :command-name="linesCommand"
      />
      <ProfileStatsPanel
        :title="$t('ui.profile.blitz')"
        :command-name="blitzCommand"
      />
    </div>
  </BackProfilePage>
</template>

<style scoped>
div#holder-stats-profile {
  display: flex;
  align-items: center;
  justify-content: center;
  flex-direction: row;
  width: 100%;
  height: 100%;
}
</style>
<script setup lang="ts">
import { useI18n } from "vue-i18n";
import ProfileStatsPanel from "../components/ProfileStatsPanel.vue";
import BackProfilePage from "../layouts/BackProfilePage.vue";
useI18n();
const classicCommand = "retreive_classic_game_info";
const linesCommand = "retreive_lines_game_info";
const blitzCommand = "retreive_blitz_game_info";
</script>
