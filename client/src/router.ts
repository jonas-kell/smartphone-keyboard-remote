import { createWebHashHistory, createRouter } from "vue-router";

import Index from "./components/Index.vue";
import Configure from "./components/Configure.vue";

const routes = [
    { path: "/", component: Index, name: "index" },
    { path: "/configure", component: Configure, name: "configure" },
    { path: "/:catchAll(.*)*", component: Index },
];

export default createRouter({
    history: createWebHashHistory(),
    routes,
});
