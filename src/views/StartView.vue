<template>
  <section class="page" aria-label="主页面">
    <div class="bg" />

    <div class="content">
      <h1 class="title">Stardew Valley</h1>
      <p class="desc">一键启动星露谷物语</p>
    </div>

    <div class="actions">
      <button class="start" type="button" :disabled="starting" @click="onStart">
        {{ starting ? "启动中…" : "开始游戏" }}
      </button>
      <p v-if="error" class="error" role="status">{{ error }}</p>
    </div>
  </section>
</template>

<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { ref } from "vue";

const starting = ref(false);
const error = ref<string | null>(null);

async function onStart() {
  if (starting.value) return;
  starting.value = true;
  error.value = null;

  try {
    await invoke("start_game");
  } catch (e) {
    error.value = e instanceof Error ? e.message : String(e);
  } finally {
    starting.value = false;
  }
}
</script>

<style scoped>
.page {
  position: relative;
  margin: -20px;
  width: calc(100% + 40px);
  height: calc(100% + 40px);
  min-height: calc(100% + 40px);
  overflow: hidden;
  border-radius: 0;
}

.bg {
  position: absolute;
  inset: 0;
  background:
    linear-gradient(180deg, rgba(0, 0, 0, 0.25), rgba(0, 0, 0, 0.65)),
    url("/start-bg.jpg") center / cover no-repeat;
  filter: saturate(1.05);
  transform: scale(1.02);
}

.content {
  position: relative;
  padding: 24px;
  padding-top: 100px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.title {
  margin: 0;
  font-size: 44px;
  letter-spacing: 0.2px;
  color: rgba(255, 255, 255, 0.95);
}

.desc {
  margin: 0;
  font-size: 18px;
  line-height: 1.4;
  color: rgba(255, 255, 255, 0.88);
}

.actions {
  position: absolute;
  right: 28px;
  bottom: 28px;
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: 8px;
}

.start {
  width: auto;
  min-width: 180px;
  max-width: min(360px, calc(100vw - 120px));
  border: 0;
  border-radius: 16px;
  padding: 16px 18px;
  font-weight: 700;
  font-size: 18px;
  letter-spacing: 0.2px;
  color: rgba(255, 255, 255, 0.92);
  background: rgba(255, 255, 255, 0.16);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  box-shadow:
    0 10px 24px rgba(0, 0, 0, 0.3),
    inset 0 1px 0 rgba(255, 255, 255, 0.12);
  cursor: pointer;
}

.start:hover:enabled {
  background: rgba(255, 255, 255, 0.2);
}

.start:active:enabled {
  transform: translateY(1px);
}

.start:disabled {
  cursor: not-allowed;
  opacity: 0.7;
}

.error {
  margin: 0;
  max-width: min(60vw, 520px);
  padding: 6px 10px;
  border-radius: 10px;
  color: rgba(255, 255, 255, 0.92);
  background: rgba(180, 48, 48, 0.45);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
}
</style>
