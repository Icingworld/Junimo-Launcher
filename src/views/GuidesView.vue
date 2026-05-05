<script setup lang="ts">
import { openUrl } from "@tauri-apps/plugin-opener";

type Guide = {
  name: string;
  description: string;
  url: string;
  logo: string;
};

const guides: Guide[] = [
  {
    name: "Steam",
    description: "Steam 官方网站",
    url: "https://store.steampowered.com/",
    logo: "/steam_logo.png",
  },
  {
    name: "星露谷物语中文 Wiki",
    description: "最全面的星露谷物语中文百科",
    url: "https://zh.stardewvalleywiki.com/Stardew_Valley_Wiki",
    logo: "/stardew_valley_wiki_logo.png",
  },
  {
    name: "SMAPI 模组管理器",
    description: "模组框架 SMAPI 的下载站",
    url: "https://smapi.io/",
    logo: "/smapi_logo.png",
  },
  {
    name: "Nexus Mods",
    description: "最大的星露谷模组下载平台",
    url: "https://www.nexusmods.com/stardewvalley",
    logo: "/nexusmods_logo.png",
  },
];

async function openGuide(guide: Guide) {
  await openUrl(guide.url);
}
</script>

<template>
  <section class="guides-view">
    <header class="guides-header">
      <h1 class="title">常用游戏资源与攻略站点</h1>
    </header>

    <div class="cards">
      <article
        v-for="guide in guides"
        :key="guide.url"
        class="card"
        role="button"
        tabindex="0"
        @click="openGuide(guide)"
        @keydown.enter="openGuide(guide)"
        @keydown.space.prevent="openGuide(guide)"
      >
        <div class="card-logo">
          <img :src="guide.logo" :alt="`${guide.name} logo`" loading="lazy" />
        </div>
        <div class="card-body">
          <h2 class="card-name">{{ guide.name }}</h2>
          <p class="card-desc">{{ guide.description }}</p>
        </div>
        <div class="card-arrow" aria-hidden="true">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
            <path d="M5 12h14M12 5l7 7-7 7" />
          </svg>
        </div>
      </article>
    </div>
  </section>
</template>

<style scoped>
.guides-view {
  display: flex;
  flex-direction: column;
  gap: 20px;
  min-width: 0;
  height: 100%;
  overflow: auto;
}

.guides-header {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.title {
  margin: 0;
  font-size: 20px;
  font-weight: 700;
}

.cards {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 14px;
}

.card {
  position: relative;
  display: flex;
  align-items: center;
  gap: 14px;
  padding: 16px;
  border-radius: 14px;
  background: rgba(255, 255, 255, 0.5);
  backdrop-filter: blur(8px);
  -webkit-backdrop-filter: blur(8px);
  box-shadow:
    0 2px 12px rgba(0, 0, 0, 0.06),
    inset 0 1px 0 rgba(255, 255, 255, 0.7);
  border: 1px solid rgba(255, 255, 255, 0.4);
  cursor: pointer;
  transition: transform 120ms ease, box-shadow 120ms ease, background 120ms ease;
  user-select: none;
}

.card:hover {
  background: rgba(255, 255, 255, 0.75);
  box-shadow:
    0 6px 20px rgba(0, 0, 0, 0.1),
    inset 0 1px 0 rgba(255, 255, 255, 0.8);
  transform: translateY(-2px);
}

.card:active {
  transform: translateY(0);
  box-shadow:
    0 2px 8px rgba(0, 0, 0, 0.06),
    inset 0 1px 0 rgba(255, 255, 255, 0.6);
}

.card:focus-visible {
  outline: 2px solid rgba(100, 108, 255, 0.6);
  outline-offset: 2px;
}

.card-logo {
  flex-shrink: 0;
  width: 48px;
  height: 48px;
  border-radius: 10px;
  overflow: hidden;
  background: #fff;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
}

.card-logo img {
  width: 100%;
  height: 100%;
  object-fit: contain;
}

.card-body {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.card-name {
  margin: 0;
  font-size: 15px;
  font-weight: 700;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.card-desc {
  margin: 0;
  font-size: 13px;
  line-height: 1.5;
  opacity: 0.65;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  line-clamp: 2;
  overflow: hidden;
}

.card-arrow {
  flex-shrink: 0;
  color: rgba(100, 108, 255, 0.6);
  transition: color 120ms ease, transform 120ms ease;
}

.card:hover .card-arrow {
  color: rgba(100, 108, 255, 0.9);
  transform: translateX(3px);
}

@media (prefers-color-scheme: dark) {
  .card {
    background: rgba(255, 255, 255, 0.08);
    box-shadow:
      0 2px 12px rgba(0, 0, 0, 0.25),
      inset 0 1px 0 rgba(255, 255, 255, 0.1);
    border-color: rgba(255, 255, 255, 0.1);
  }

  .card:hover {
    background: rgba(255, 255, 255, 0.12);
    box-shadow:
      0 6px 20px rgba(0, 0, 0, 0.35),
      inset 0 1px 0 rgba(255, 255, 255, 0.15);
  }

  .card-logo {
    background: rgba(255, 255, 255, 0.1);
  }
}
</style>
