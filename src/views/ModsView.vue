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
  minimum_api_version: string | null;
  update_keys: string;
  enabled: boolean;
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

const detailVisible = ref(false);
const detailMod = ref<ModDto | null>(null);

const deleteVisible = ref(false);
const pendingDelete = ref<ModDto | null>(null);

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

function openDetail(row: ModDto) {
  detailMod.value = row;
  detailVisible.value = true;
}

function openDeleteDialog(row: ModDto) {
  pendingDelete.value = row;
  deleteVisible.value = true;
}

async function confirmDelete() {
  const row = pendingDelete.value;
  if (!row) return;
  try {
    await invoke("mods_delete", { modId: row.id });
    ElMessage.success("已删除");
    if (detailMod.value?.id === row.id) {
      detailVisible.value = false;
      detailMod.value = null;
    }
    deleteVisible.value = false;
    pendingDelete.value = null;
    await refresh();
  } catch (e) {
    ElMessage.error(e instanceof Error ? e.message : String(e));
  }
}

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
      title: "选择模组文件夹",
    });
    if (!selected) return;
    const folderPath = Array.isArray(selected) ? selected[0] : selected;
    if (!folderPath) return;

    await invoke("mods_add", { folderPath });
    ElMessage.success("添加成功");
    page.value = 1;
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
    if (detailVisible.value && detailMod.value?.id === row.id) {
      detailMod.value = { ...detailMod.value, enabled: next };
    }
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
  <section class="mods-view">
    <div class="mods-toolbar">
      <ElSpace wrap>
        <ElInput v-model="nameLike" clearable placeholder="按模组名称搜索" />
        <div class="mods-filter">
          <ElSelect v-model="enabledFilter" class="mods-filter-select">
            <ElOption label="全部" value="all" />
            <ElOption label="已启用" value="enabled" />
            <ElOption label="未启用" value="disabled" />
          </ElSelect>
        </div>
      </ElSpace>
      <ElButton type="primary" @click="onAddMod">添加模组</ElButton>
    </div>

    <ElTable :data="items" v-loading="loading" row-key="id">
      <ElTableColumn prop="name" label="名称" min-width="160" show-overflow-tooltip />
      <ElTableColumn prop="version" label="版本" width="100" />

      <ElTableColumn label="状态" fixed="right" width="88" align="center">
        <template #default="{ row }">
          <ElSwitch
            :model-value="row.enabled"
            :disabled="!hasGamePath"
            @change="(v: boolean) => onToggleEnabled(row, v)"
          />
        </template>
      </ElTableColumn>

      <ElTableColumn label="操作" fixed="right" width="200" align="center">
        <template #default="{ row }">
          <ElButton type="primary" link @click="openDetail(row)">详情</ElButton>
          <ElButton type="danger" link @click="openDeleteDialog(row)">删除</ElButton>
        </template>
      </ElTableColumn>
    </ElTable>

    <div class="mods-pager">
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

    <ElDialog v-model="detailVisible" title="模组详情" width="560px" destroy-on-close>
      <div v-if="detailMod" class="mod-detail-body">
        <ElDescriptions :column="1" border>
          <ElDescriptionsItem label="名称">
            <div class="mod-detail-value">{{ detailMod.name }}</div>
          </ElDescriptionsItem>
          <ElDescriptionsItem label="版本">
            <div class="mod-detail-value">{{ detailMod.version }}</div>
          </ElDescriptionsItem>
          <ElDescriptionsItem label="模组ID">
            <div class="mod-detail-value">{{ detailMod.unique_id }}</div>
          </ElDescriptionsItem>
          <ElDescriptionsItem label="作者">
            <div class="mod-detail-value">{{ detailMod.author || "-" }}</div>
          </ElDescriptionsItem>
          <ElDescriptionsItem label="描述">
            <div class="mod-detail-value">{{ detailMod.description || "-" }}</div>
          </ElDescriptionsItem>
          <ElDescriptionsItem label="最小SMAPI版本">
            <div class="mod-detail-value">{{ detailMod.minimum_api_version || "-" }}</div>
          </ElDescriptionsItem>
          <ElDescriptionsItem label="更新方式">
            <div class="mod-detail-value">{{ detailMod.update_keys }}</div>
          </ElDescriptionsItem>
          <ElDescriptionsItem label="状态">
            <div class="mod-detail-value">{{ detailMod.enabled ? "已启用" : "未启用" }}</div>
          </ElDescriptionsItem>
        </ElDescriptions>
      </div>
      <template #footer>
        <ElButton type="primary" @click="detailVisible = false">关闭</ElButton>
      </template>
    </ElDialog>

    <ElDialog
      v-model="deleteVisible"
      title="确认删除"
      width="440px"
      destroy-on-close
      @closed="pendingDelete = null"
    >
      <p>
        确定删除模组「{{ pendingDelete?.name }}」？将同时删除应用数据目录中的存储文件夹，此操作不可恢复。
      </p>
      <template #footer>
        <ElButton @click="deleteVisible = false">取消</ElButton>
        <ElButton type="danger" @click="confirmDelete">删除</ElButton>
      </template>
    </ElDialog>
  </section>
</template>

<style scoped>
.mods-view {
  display: flex;
  flex-direction: column;
  gap: 16px;
  min-width: 0;
}

.mods-toolbar {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
}

.mods-filter {
  display: inline-flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 8px;
}

.mods-filter-prefix {
  font-size: 14px;
  color: var(--el-text-color-regular);
  white-space: nowrap;
}

.mods-filter-select {
  width: 128px;
}

.mods-pager {
  display: flex;
  justify-content: flex-end;
  flex-wrap: wrap;
}

/* 详情弹窗：避免 ElDescriptions 表格被长文本撑出弹窗 */
.mod-detail-body :deep(.el-descriptions__body table) {
  table-layout: fixed;
  width: 100%;
}

.mod-detail-body :deep(.el-descriptions__label) {
  width: 148px;
  vertical-align: top;
}

.mod-detail-body :deep(.el-descriptions__content) {
  vertical-align: top;
}

.mod-detail-value {
  min-width: 0;
  word-break: break-word;
  overflow-wrap: anywhere;
}
</style>
