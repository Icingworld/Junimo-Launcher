<template>
  <RouterView />
</template>

<style>
:root {
  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

html,
body,
#app {
  height: 100%;
  margin: 0;
}

body {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  color: #0f0f0f;
  background-color: #f6f6f6;
}

/* Element Plus tooltip: remove fade in/out delays */
.el-popper.is-tooltip,
.el-tooltip__popper {
  transition-duration: 0ms !important;
  animation-duration: 0ms !important;
}

.el-fade-in-linear-enter-active,
.el-fade-in-linear-leave-active,
.el-zoom-in-top-enter-active,
.el-zoom-in-top-leave-active,
.el-zoom-in-bottom-enter-active,
.el-zoom-in-bottom-leave-active,
.el-zoom-in-left-enter-active,
.el-zoom-in-left-leave-active,
.el-zoom-in-right-enter-active,
.el-zoom-in-right-leave-active {
  transition-duration: 0ms !important;
  animation-duration: 0ms !important;
}

@media (prefers-color-scheme: dark) {
  body {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }
}
</style>