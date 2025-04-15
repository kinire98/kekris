<template>
  <div id="logo">
    <img src="/logo-drop-shadow.png" alt="" />
    <h2>{{ $t("ui.index.click") }}</h2>
  </div>
  <div id="languages">
    <select id="languages-select">
      <option
        :value="language"
        @click="changeLocale(language)"
        v-for="language in availableLocales"
      >
        {{ $t("languages." + language) }}
      </option>
    </select>
  </div>
  <div id="front"></div>
  <Dialog v-model:visible="visible" modal :header="$t('ui.username.select')">
    <InputText type="text" v-model="value" :invalid="!value" />
    <Button
      :label="$t('ui.username.confirm')"
      variant="outlined"
      raised
      @click="onConfirmed"
    ></Button>
  </Dialog>
</template>
<style scoped>
* {
  cursor: pointer;
}
#languages {
  position: absolute;
  bottom: 0;
  right: 0;
  z-index: 3;
}
#logo {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 95vh;
  width: 100vw;
  flex-direction: column;
  * {
    margin-block: 5rem;
  }
}
#front {
  position: absolute;
  top: 0;
  bottom: 0;
  left: 0;
  right: 0;
  z-index: 2;
}
h2 {
  animation: opa 2s ease-in infinite;
}
@keyframes opa {
  0% {
    opacity: 1;
  }
  50% {
    opacity: 0;
  }
  100% {
    opacity: 1;
  }
}
.p-button-outlined {
  border: 2.5px solid var(--main-color) !important;
  color: var(--main-color) !important;
  margin-block: 10px;
  transition: all 0.3s;
  display: flex;
  align-items: center;
  justify-content: start;
}
.p-button-outlined:hover {
  border-color: var(--main-contrast) !important;
  color: var(--main-contrast) !important;
  background-color: var(--main-color) !important;
}
.p-inputtext {
  border: 1px solid var(--transparent-main-color);
  margin-bottom: 5px;
}
.p-inputtext:enabled:focus {
  border: 1px solid var(--main-color);
}
</style>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useRouter } from "vue-router";
import { useI18n } from "vue-i18n";
import Dialog from "primevue/dialog";
import Button from "primevue/button";
import InputText from "primevue/inputtext";
import { getUsername, setUsername } from "../helpers/username";

const { locale, availableLocales, t } = useI18n();

const visible = ref(false);
const value = ref("");

const router = useRouter();

function changeLocale(localeString: string) {
  locale.value = localeString;
}

function changeRouteChecking() {
  const username = getUsername();
  if (!username) {
    visible.value = true;
  } else {
    router.push("/main");
  }
}

function onConfirmed() {
  if (value.value.length > 0) {
    setUsername(value.value);
    router.push("/main");
  }
}

onMounted(() => {
  const front = document.getElementById("front");
  if (front) {
    front.addEventListener("click", changeRouteChecking);
  }
});
</script>
