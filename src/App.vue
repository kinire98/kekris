<template>
  <Suspense>
    <RouterView />
  </Suspense>
  <div id="bgc"></div>
</template>

<style>
@font-face {
  font-family: "audiowide";
  src: url("/fonts/Audiowide-Regular.ttf");
}

:root {
  font-family: audiowide;
  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
  --main-color: #00adb5;
  --transparent-main-color: #00adb522;
  --main-contrast: #222831;
  --white-constrast: #eeeeee;
  --black-constrast: #000000;
  --black-constrast-transparent: #00000044;
}

html,
body {
  margin: 0;
  padding: 0;
  height: 100%;
  min-height: 100%;
  min-width: 100%;
  color: var(--white-constrast);
  background-color: var(--black-constrast);
  overflow: hidden;
}
* {
  -webkit-touch-callout: none;
  -webkit-user-select: none;
  -khtml-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;
}
/* For WebKit browsers */
::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

::-webkit-scrollbar-track {
  background: transparent;
}

::-webkit-scrollbar-thumb {
  background-color: rgba(0, 0, 0, 0.3);
  border-radius: 4px;
  border: 2px solid transparent;
  background-clip: content-box;
}

::-webkit-scrollbar-thumb:hover {
  background-color: rgba(0, 0, 0, 0.5);
}

#bgc {
  position: absolute;
  top: 0;
  bottom: 0;
  left: 0;
  right: 0;
  background-image: radial-gradient(transparent, var(--transparent-main-color));
  transition: all 0.3s;
  z-index: 0 !important;
}
* {
  z-index: 2 !important;
}
h1 {
  text-align: center;
}
</style>
<script>
import forbidBack from "./helpers/forbidBack";
import forbidReload from "./helpers/forbidReload";

forbidBack();
forbidReload();
</script>
