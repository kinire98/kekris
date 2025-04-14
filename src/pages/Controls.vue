<template>
  <MenuBackLayout :title="$t('ui.controls.controls')" back>
    <div id="codes">
      <SelectKeyCode
        :info="$t('ui.controls.right')"
        :movementKey="rightMoveCodeKey"
        :value="getRightMoveCode()"
      />
      <SelectKeyCode
        :info="$t('ui.controls.left')"
        :movementKey="leftMoveCodeKey"
        :value="getLeftMoveCode()"
      />
      <SelectKeyCode
        :info="$t('ui.controls.hard-drop')"
        :movementKey="hardDropCodeKey"
        :value="getHardDropCode()"
      />
      <SelectKeyCode
        :info="$t('ui.controls.soft-drop')"
        :movementKey="softDropCodeKey"
        :value="getSoftDropCode()"
      />
    </div>
  </MenuBackLayout>
</template>
<style scoped>
div#codes {
  width: 75%;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-direction: column;
}
</style>
<script setup>
import {
  getHardDropCode,
  getLeftMoveCode,
  getRightMoveCode,
  getSoftDropCode,
  hardDropCodeKey,
  leftMoveCodeKey,
  rightMoveCodeKey,
  softDropCodeKey,
} from "../controls/keycodes";
</script>
<script>
import { useI18n } from "vue-i18n";
import MenuBackLayout from "../layouts/MenuBackLayout.vue";
export default {
  setup() {
    const t = useI18n();
    return t;
  },
};
</script>
