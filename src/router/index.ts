import { createRouter, createWebHashHistory } from "vue-router";

import MainLayout from "../layouts/MainLayout.vue";

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    {
      path: "/",
      component: MainLayout,
      redirect: "/start",
      children: [
        {
          path: "start",
          name: "start",
          component: () => import("../views/StartView.vue"),
        },
        {
          path: "mods",
          name: "mods",
          component: () => import("../views/ModsView.vue"),
        },
        {
          path: "guides",
          name: "guides",
          component: () => import("../views/GuidesView.vue"),
        },
        {
          path: "settings",
          name: "settings",
          component: () => import("../views/SettingsView.vue"),
        },
      ],
    },
  ],
});

export default router;
