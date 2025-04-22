<template>
  <div class="root">
    <span class="info">
      {{ props.name }}
    </span>
    <span class="value">
      <span class="icon-holder">
        <Icon
          icon="teenyicons:anti-clockwise-solid"
          class="again-icon"
          @click="resetValue"
        />
      </span>
      <span>{{ value + "ms" }}</span>
      <Slider v-model="value" :min="Number(props.min)" :max="Number(props.max)"
    /></span>
    <span
      id="description"
      v-if="props.desc != null"
      :title="props.desc.toString()"
    >
      <Icon
        icon="iconamoon:question-mark-circle-duotone"
        class="icon-question-controls"
      />
    </span>
  </div>
</template>
<style>
.p-slider-horizontal .p-slider-range {
  background-color: var(--main-color) !important;
}
</style>
<style scoped>
div {
  display: flex;
  align-items: center;
  justify-content: space-evenly;
  width: 85%;
  background-color: #30303066;
  padding: 15px;
  border-radius: 15px;
  margin-block: 15px;
  * {
    width: 85%;
    display: flex;
    align-items: center;
    justify-content: center;
    text-align: center;
  }
}
#description {
  width: 5vw;
  height: 1.5vw;
  margin-left: 15px;
  * {
    width: 100%;
    height: 100%;
  }
}
#value {
  background-color: #30303066;
  padding: 10px;
  border-radius: 15px;
  cursor: pointer;
  transition: all 0.3s;
}
#value:hover {
  transform: scale(1.05);
}
#info {
  margin-right: 15px;
}
.icon-holder {
  width: 25px;
}
.again-icon {
  color: var(--main-color);
  font-size: 25px;
  transition: all 0.3s;
  cursor: pointer;
  width: 25px;
}
.again-icon:hover {
  transform: scale(1.2);
}
</style>
<script setup lang="ts">
import { Icon } from "@iconify/vue/dist/iconify.js";
import { Slider } from "primevue";
import { ref, watch } from "vue";
const props = defineProps<{
  name: String;
  initialValue: Number;
  defaultValue: Number;
  desc: String;
  min: Number;
  max: Number;
}>();
let value = ref(Number(props.initialValue));
const emit = defineEmits(["change"]);
watch(value, (newValue, _oldValue) => {
  emit("change", newValue);
});
function resetValue() {
  value.value = Number(props.defaultValue);
}
</script>
