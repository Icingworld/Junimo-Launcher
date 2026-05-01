<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { ElMessage } from "element-plus";
import { computed, onMounted, ref, watch } from "vue";

type EnabledFilter = "all" | "enabled" | "disabled";

type ModDto = {
  id: number;
  unique_id: string;
  version: string;
  name: string;
  author: string | null;
  description: string | null;
  enabled: boolean;
  source_folder_name: string;
  created_at: number;
};

type PagedMods = {
  total: number;
  items: ModDto[];
};

const loading = ref(false);
const items = ref<ModDto[]>([]);
const total = ref(0);

const page = ref(1);
const pageSize = ref(10);

const nameLike = ref("");
const enabledFilter = ref<EnabledFilter>("all");

const gamePath = ref<string | null>(null);
const hasGamePath = computed(() => !!gamePath.value);

let searchTimer: number | null = null;
watch(nameLike, () => {
  if (searchTimer) window.clearTimeout(searchTimer);
  searchTimer = window.setTimeout(() => {
    page.value = 1;
    void refresh();
  }, 250);
});

watch(enabledFilter, () => {
  page.value = 1;
  void refresh();
});

async function refresh() {
  if (loading.value) return;
  loading.value = true;
  try {
    const enabled_filter =
      enabledFilter.value === "enabled" ? 1 : enabledFilter.value === "disabled" ? 0 : -1;
    const res = await invoke<PagedMods>("mods_list", {
      page: page.value,
      pageSize: pageSize.value,
      nameLike: nameLike.value.trim() || null,
      enabledFilter: enabled_filter,
    });
    items.value = res.items;
    total.value = res.total;
  } catch (e) {
    ElMessage.error(e instanceof Error ? e.message : String(e));
  } finally {
    loading.value = false;
  }
}

async function refreshGamePath() {
  try {
    gamePath.value = await invoke<string | null>("settings_get_game_path");
  } catch {
    gamePath.value = null;
  }
}

async function onAddMod() {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: "选择模组文件夹（包含 manifest.json）",
    });
    if (!selected) return;
    const folderPath = Array.isArray(selected) ? selected[0] : selected;
    if (!folderPath) return;

    await invoke("mods_add", { folderPath });
    ElMessage.success("添加成功，默认已禁用");
    page.value = 1;
    await refresh();
  } catch (e) {
    ElMessage.error(e instanceof Error ? e.message : String(e));
  }
}

async function onDelete(row: ModDto) {
  try {
    await invoke("mods_delete", { modId: row.id });
    ElMessage.success("已删除");
    await refresh();
  } catch (e) {
    ElMessage.error(e instanceof Error ? e.message : String(e));
  }
}

async function onToggleEnabled(row: ModDto, next: boolean) {
  if (!hasGamePath.value) {
    ElMessage.warning("请先在设置中选择游戏目录，再启用/禁用模组");
    await refreshGamePath();
    await refresh();
    return;
  }

  try {
    await invoke("mods_set_enabled", { modId: row.id, enabled: next });
    row.enabled = next;
    ElMessage.success(next ? "已启用" : "已禁用");
  } catch (e) {
    ElMessage.error(e instanceof Error ? e.message : String(e));
    await refreshGamePath();
    await refresh();
  }
}

onMounted(async () => {
  await refreshGamePath();
  await refresh();
});
</script>

<template>
  <section class="page">
    <div class="toolbar">
      <ElInput
        v-model="nameLike"
        clearable
        placeholder="按模组名称搜索"
        class="search"
      />

      <ElSelect v-model="enabledFilter" class="filter" placeholder="状态">
        <ElOption label="全部" value="all" />
        <ElOption label="已启用" value="enabled" />
        <ElOption label="未启用" value="disabled" />
      </ElSelect>

      <ElButton type="primary" @click="onAddMod">添加模组</ElButton>
    </div>

    <ElAlert
      v-if="!hasGamePath"
      type="warning"
      :closable="false"
      show-icon
      class="hint"
      title="尚未设置游戏目录：启用/禁用功能将被限制"
      description="请前往「设置」选择 Stardew Valley 游戏目录。"
    />

    <ElTable :data="items" v-loading="loading" class="table" row-key="id">
      <ElTableColumn prop="name" label="名称" min-width="160" />
      <ElTableColumn prop="author" label="作者" min-width="120">
        <template #default="{ row }">
          {{ row.author || "-" }}
        </template>
      </ElTableColumn>
      <ElTableColumn prop="version" label="版本" min-width="100" />
      <ElTableColumn prop="description" label="描述" min-width="240" show-overflow-tooltip>
        <template #default="{ row }">
          {{ row.description || "-" }}
        </template>
      </ElTableColumn>

      <ElTableColumn label="状态" min-width="120">
        <template #default="{ row }">
          <ElSwitch
            :model-value="row.enabled"
            :disabled="!hasGamePath"
            inline-prompt
            active-text="启用"
            inactive-text="禁用"
            @change="(v: boolean) => onToggleEnabled(row, v)"
          />
        </template>
      </ElTableColumn>

      <ElTableColumn label="操作" min-width="120" fixed="right">
        <template #default="{ row }">
          <ElPopconfirm
            title="确认删除该模组？（将删除存储文件夹）"
            confirm-button-text="删除"
            cancel-button-text="取消"
            @confirm="onDelete(row)"
          >
            <template #reference>
              <ElButton type="danger" text>删除</ElButton>
            </template>
          </ElPopconfirm>
        </template>
      </ElTableColumn>
    </ElTable>

    <div class="pager">
      <ElPagination
        v-model:current-page="page"
        v-model:page-size="pageSize"
        :total="total"
        :page-sizes="[10, 20, 50]"
        layout="total, sizes, prev, pager, next, jumper"
        @current-change="refresh"
        @size-change="
          () => {
            page = 1;
            refresh();
          }
        "
      />
    </div>
  </section>
</template>

<style scoped>
.page {
  display: flex;
  flex-direction: column;
  gap: 12px;
  min-width: 0;
}

.toolbar {
  display: grid;
  grid-template-columns: 1fr 160px auto;
  gap: 12px;
  align-items: center;
}

.search {
  min-width: 220px;
}

.filter {
  width: 160px;
}

.hint {
  margin-top: 2px;
}

.table {
  width: 100%;
}

.pager {
  display: flex;
  justify-content: flex-end;
  padding-top: 6px;
}

@media (max-width: 720px) {
  .toolbar {
    grid-template-columns: 1fr;
  }
  .filter {
    width: 100%;
  }
}
</style>
