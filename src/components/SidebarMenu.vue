<script setup lang="ts">
import { computed } from "vue";
import { useRoute, useRouter } from "vue-router";
import { Icon } from "@iconify/vue";

const props = defineProps<{
  collapsed: boolean;
}>();

const emit = defineEmits<{
  (e: "toggle-collapsed"): void;
}>();

type MenuItem = { label: string; to: string; icon: string };

const items: MenuItem[] = [
  { label: "开始", to: "/start", icon: "lucide:play" },
  { label: "模组", to: "/mods", icon: "lucide:puzzle" },
  { label: "攻略", to: "/guides", icon: "lucide:book-open" },
  { label: "设置", to: "/settings", icon: "lucide:settings" },
];

const route = useRoute();
const router = useRouter();

const activeIndex = computed(() => route.path);

function onSelect(index: string) {
  void router.push(index);
}

const isActive = (to: string) => activeIndex.value === to;
</script>

<template>
  <nav class="menu" :class="{ collapsed: props.collapsed }">
    <div class="header">
      <div class="brand" :class="{ hidden: props.collapsed }">Junimo Launcher</div>

      <ElTooltip
        :content="props.collapsed ? '展开菜单' : '收起菜单'"
        placement="right"
        :show-after="0"
        :hide-after="0"
        transition=""
      >
        <ElButton
          class="collapseBtn"
          text
          @click="emit('toggle-collapsed')"
          aria-label="切换侧边栏收起状态"
        >
          <Icon
            class="collapseIcon"
            :icon="props.collapsed ? 'lucide:chevron-right' : 'lucide:chevron-left'"
            aria-hidden="true"
          />
        </ElButton>
      </ElTooltip>
    </div>

    <ul class="list">
      <li v-for="item in items" :key="item.to" class="item">
        <ElTooltip
          v-if="props.collapsed"
          :content="item.label"
          placement="right"
          :show-after="0"
          :hide-after="0"
          transition=""
        >
          <RouterLink class="link" :to="item.to" draggable="false">
            <ElButton
              class="menuBtn"
              text
              :class="{ active: isActive(item.to) }"
              @click="onSelect(item.to)"
            >
              <Icon class="icon" :icon="item.icon" aria-hidden="true" />
              <span class="label hidden">{{ item.label }}</span>
            </ElButton>
          </RouterLink>
        </ElTooltip>

        <RouterLink v-else class="link" :to="item.to">
          <ElButton
            class="menuBtn"
            text
            :class="{ active: isActive(item.to) }"
            @click="onSelect(item.to)"
          >
            <Icon class="icon" :icon="item.icon" aria-hidden="true" />
            <span class="label">{{ item.label }}</span>
          </ElButton>
        </RouterLink>
      </li>
    </ul>
  </nav>
</template>

<style scoped>
.menu {
  display: flex;
  flex-direction: column;
  gap: 12px;
  padding: 16px;
}

.header {
  display: flex;
  align-items: center;
  gap: 8px;
  justify-content: space-between;
}

.brand {
  font-size: 14px;
  font-weight: 700;
  letter-spacing: 0.2px;
  opacity: 0.9;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.collapseBtn {
  margin-left: auto;
  width: calc(72px - 24px);
  height: 40px;
  border-radius: 10px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  padding: 0;
  color: inherit;
}

.collapseBtn:hover {
  background: rgba(0, 0, 0, 0.06);
}

.collapseBtn:active {
  transform: translateY(1px);
}

.collapseIcon {
  width: 18px;
  height: 18px;
  opacity: 0.9;
}

.list {
  list-style: none;
  margin: 0;
  padding: 0;
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.link {
  display: block;
  text-decoration: none;
  color: inherit;
  -webkit-user-drag: none;
}

.icon {
  width: 18px;
  height: 18px;
  opacity: 0.9;
}

.menuBtn {
  width: 100%;
  height: 40px;
  border-radius: 10px;
  display: inline-flex;
  align-items: center;
  justify-content: flex-start;
  padding: 0 12px;
  color: inherit;
}

.menuBtn:hover {
  background: rgba(0, 0, 0, 0.06);
}

.menuBtn:active {
  transform: translateY(1px);
}

.menuBtn.active {
  background: rgba(100, 108, 255, 0.14);
}

.label {
  font-size: 14px;
  font-weight: 600;
  margin-left: 10px;
}

.hidden {
  display: none;
}

.menu.collapsed {
  padding-left: 12px;
  padding-right: 12px;
}

.menu.collapsed .menuBtn {
  justify-content: center;
  padding: 0;
}

.menu.collapsed .header {
  justify-content: center;
}

.menu.collapsed .collapseBtn {
  margin-left: 0;
}
</style>
