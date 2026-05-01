<script setup lang="ts">
import { ref } from "vue";
import SidebarMenu from "../components/SidebarMenu.vue";

const isSidebarCollapsed = ref(false);

function toggleSidebar() {
  isSidebarCollapsed.value = !isSidebarCollapsed.value;
}
</script>

<template>
  <div class="layout" :class="{ collapsed: isSidebarCollapsed }">
    <aside class="sidebar" aria-label="侧边栏菜单">
      <div class="sidebarInner">
        <ElScrollbar class="scroll">
          <SidebarMenu
            :collapsed="isSidebarCollapsed"
            @toggle-collapsed="toggleSidebar"
          />
        </ElScrollbar>
      </div>
    </aside>

    <main class="content" aria-label="主体内容">
      <div class="contentInner">
        <RouterView />
      </div>
    </main>
  </div>
</template>

<style scoped>
.layout {
  height: 100vh;
  height: 100dvh;
  width: 100vw;
  width: 100dvw;
  display: grid;
  grid-template-columns: auto 1fr;
  min-width: 0;
}

.sidebar {
  background: rgba(255, 255, 255, 0.6);
  backdrop-filter: blur(8px);
  box-shadow: inset -1px 0 0 rgba(0, 0, 0, 0.08);
  overflow: hidden;
}

.sidebarInner {
  width: 240px;
  height: 100%;
  transition: width 180ms ease;
  will-change: width;
}

.layout.collapsed .sidebarInner {
  width: 72px;
}

.content {
  min-width: 0;
  box-sizing: border-box;
  height: 100%;
  width: 100%;
  padding: 20px;
  overflow: auto;
  display: flex;
}

.contentInner {
  box-sizing: border-box;
  flex: 1;
  min-width: 0;
  min-height: 100%;
  width: 100%;
}

@media (prefers-color-scheme: dark) {
  .sidebar {
    background: rgba(0, 0, 0, 0.2);
    box-shadow: inset -1px 0 0 rgba(255, 255, 255, 0.12);
  }
}
</style>
