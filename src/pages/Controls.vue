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
      <SelectKeyCode
        :info="$t('ui.controls.save-piece')"
        :movementKey="savePieceCodeKey"
        :value="getSavePieceCode()"
      />
      <SelectKeyCode
        :info="$t('ui.controls.clockwise')"
        :movementKey="clockwiseCodeKey"
        :value="getClockwiseCode()"
      />
      <SelectKeyCode
        :info="$t('ui.controls.counter-clockwise')"
        :movementKey="counterClockwiseCodeKey"
        :value="getCounterClockwiseCode()"
      />
      <SelectKeyCode
        :info="$t('ui.controls.full-rotation')"
        :movementKey="fullRotationCodeKey"
        :value="getFullRotationCode()"
      />
      <SelectKeyCode
        :info="$t('ui.controls.forfeit')"
        :movementKey="forfeitCodeKey"
        :value="getForfeitCode()"
      />
      <SelectKeyCode
        :info="$t('ui.controls.retry')"
        :movementKey="retryCodeKey"
        :value="getRetryCode()"
      />
      <SelectKeyCode
        :info="$t('ui.controls.random')"
        :movementKey="randomCodeKey"
        :value="getTargetingRandomCode()"
        :desc="$t('ui.controls.random-desc')"
      />
      <SelectKeyCode
        :info="$t('ui.controls.even')"
        :movementKey="evenCodeKey"
        :value="getTargetingEvenCode()"
        :desc="$t('ui.controls.even-desc')"
      />
      <SelectKeyCode
        :info="$t('ui.controls.elimination')"
        :movementKey="eliminationCodeKey"
        :value="getTargetingEliminationsCode()"
        :desc="$t('ui.controls.elimination-desc')"
      />
      <SelectKeyCode
        :info="$t('ui.controls.payback')"
        :movementKey="paybackCodeKey"
        :value="getTargetingPaybackCode()"
        :desc="$t('ui.controls.payback-desc')"
      />
    </div>
  </MenuBackLayout>
</template>
<style scoped>
div#codes {
  width: 75%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: start;
  flex-direction: column;
  overflow-y: scroll;
  margin-right: 10vw;
}
</style>
<script setup lang="ts">
import {
  clockwiseCodeKey,
  counterClockwiseCodeKey,
  eliminationCodeKey,
  evenCodeKey,
  forfeitCodeKey,
  fullRotationCodeKey,
  getClockwiseCode,
  getCounterClockwiseCode,
  getForfeitCode,
  getFullRotationCode,
  getHardDropCode,
  getLeftMoveCode,
  getRetryCode,
  getRightMoveCode,
  getSavePieceCode,
  getSoftDropCode,
  getTargetingEliminationsCode,
  getTargetingEvenCode,
  getTargetingPaybackCode,
  getTargetingRandomCode,
  hardDropCodeKey,
  leftMoveCodeKey,
  paybackCodeKey,
  randomCodeKey,
  retryCodeKey,
  rightMoveCodeKey,
  savePieceCodeKey,
  softDropCodeKey,
} from "../controls/keycodes";
const t = useI18n();
</script>
<script lang="ts">
import { useI18n } from "vue-i18n";
import MenuBackLayout from "../layouts/MenuBackLayout.vue";
export default {
  mounted() {
    console.log("here");
    const $codes = document.getElementById("codes")! as HTMLElement;
    $codes.scrollTop = 0;
  },
};
</script>
