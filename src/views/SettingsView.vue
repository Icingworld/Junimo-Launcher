<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { ElMessage } from "element-plus";
import { computed, onMounted, ref } from "vue";

const gamePath = ref<string | null>(null);
const loading = ref(false);
const hasGamePath = computed(() => !!gamePath.value);

async function refresh() {
  try {
    gamePath.value = await invoke<string | null>("settings_get_game_path");
  } catch (e) {
    gamePath.value = null;
    ElMessage.error(e instanceof Error ? e.message : String(e));
  }
}

async function onPickGamePath() {
  if (loading.value) return;
  loading.value = true;
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: "选择 Stardew Valley 游戏目录",
    });
    if (!selected) return;
    const path = Array.isArray(selected) ? selected[0] : selected;
    if (!path) return;

    await invoke("settings_set_game_path", { path });
    gamePath.value = path;
    ElMessage.success("已保存游戏目录");
  } catch (e) {
    ElMessage.error(e instanceof Error ? e.message : String(e));
  } finally {
    loading.value = false;
  }
}

async function onClearGamePath() {
  if (loading.value) return;
  loading.value = true;
  try {
    await invoke("settings_set_game_path", { path: null });
    gamePath.value = null;
    ElMessage.success("已清除游戏目录");
  } catch (e) {
    ElMessage.error(e instanceof Error ? e.message : String(e));
  } finally {
    loading.value = false;
  }
}

onMounted(() => {
  void refresh();
});
</script>

<template>
  <section class="page">
    <div class="header">
      <h1 class="title">设置</h1>
      <p class="desc">管理游戏目录等基础配置（后续可扩展更多设置项）。</p>
    </div>

    <ElCard class="card" shadow="never">
      <template #header>
        <div class="cardHeader">
          <div class="cardTitle">游戏目录</div>
        </div>
      </template>

      <div class="row">
        <div class="label">Stardew Valley 路径</div>
        <div class="value">
          <ElText v-if="hasGamePath" class="path" truncated>
            {{ gamePath }}
          </ElText>
          <ElText v-else type="info">未设置</ElText>
        </div>
      </div>

      <div class="actions">
        <ElButton type="primary" :loading="loading" @click="onPickGamePath">选择目录</ElButton>
        <ElButton :disabled="!hasGamePath" :loading="loading" @click="onClearGamePath">
          清除
        </ElButton>
      </div>

      <ElAlert
        type="info"
        :closable="false"
        show-icon
        class="tip"
        title="说明"
        description="模组启用/禁用需要该目录，用于在游戏 Mods 文件夹中创建/删除链接。"
      />
    </ElCard>
  </section>
</template>

<style scoped>
.page {
  display: flex;
  flex-direction: column;
  gap: 14px;
  min-width: 0;
}

.header {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.title {
  margin: 0;
  font-size: 20px;
}

.desc {
  margin: 0;
  opacity: 0.75;
}

.card {
  border-radius: 14px;
}

.cardHeader {
  display: flex;
  align-items: center;
  justify-content: space-between;
  min-width: 0;
}

.cardTitle {
  font-weight: 700;
}

.row {
  display: grid;
  grid-template-columns: 160px 1fr;
  gap: 12px;
  align-items: center;
  min-width: 0;
}

.label {
  font-weight: 600;
  opacity: 0.85;
}

.value {
  min-width: 0;
}

.path {
  max-width: 100%;
}

.actions {
  display: flex;
  gap: 10px;
  margin-top: 14px;
}

.tip {
  margin-top: 14px;
}

@media (max-width: 720px) {
  .row {
    grid-template-columns: 1fr;
  }
}
</style>
