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
  <section class="settings-page">
    <ElCard class="settings-card" shadow="never">
      <template #header>
        <span class="section-title">游戏目录</span>
      </template>

      <div class="setting-row">
        <div class="setting-label">
          <span class="setting-label-main">Stardew Valley 路径</span>
        </div>
        <div class="setting-main">
          <p class="setting-value" :class="{ 'is-placeholder': !hasGamePath }" :title="gamePath ?? undefined">
            {{ hasGamePath ? gamePath : "未设置" }}
          </p>
          <div class="setting-actions">
            <ElButton type="primary" :loading="loading" @click="onPickGamePath">选择目录</ElButton>
            <ElButton :disabled="!hasGamePath" :loading="loading" @click="onClearGamePath">
              清除
            </ElButton>
          </div>
        </div>
      </div>
    </ElCard>
  </section>
</template>

<style scoped>
.settings-page {
  min-width: 0;
  max-width: 720px;
}

.settings-card {
  border-radius: 8px;
}

.section-title {
  font-size: 15px;
  font-weight: 600;
  color: var(--el-text-color-primary);
}

.setting-row {
  display: flex;
  align-items: center;
  gap: 20px;
  min-height: 40px;
}

.setting-label {
  flex: 0 0 168px;
  padding-top: 1px;
}

.setting-label-main {
  display: block;
  font-size: 14px;
  line-height: 22px;
  color: var(--el-text-color-regular);
}

.setting-main {
  flex: 1;
  min-width: 0;
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 16px;
}

.setting-value {
  flex: 1;
  min-width: 0;
  margin: 0;
  font-size: 14px;
  line-height: 22px;
  color: var(--el-text-color-primary);
  text-align: right;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.setting-value.is-placeholder {
  color: var(--el-text-color-placeholder);
}

.setting-actions {
  flex-shrink: 0;
  display: inline-flex;
  align-items: center;
  gap: 8px;
}

@media (max-width: 560px) {
  .setting-row {
    flex-direction: column;
    align-items: stretch;
    gap: 10px;
  }

  .setting-label {
    flex: none;
  }

  .setting-main {
    flex-direction: column;
    align-items: stretch;
    justify-content: flex-start;
  }

  .setting-value {
    text-align: left;
  }

  .setting-actions {
    justify-content: flex-start;
  }
}
</style>
