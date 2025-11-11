import { createApp } from "vue";
import { DockviewVue } from "dockview-vue";
// import { createWebHistory, createRouter } from "vue-router";

import '@/assets/dockview.scss';

import App from "./components/App.vue";
// import routes from "./routes";

// const router = createRouter({
//     history: createWebHistory(),
//     routes,
// });

import ViewFiles from "./components/views/ViewFiles.vue";
import ViewHome from "./components/views/ViewHome.vue";
import RoutePanel from "./components/dockview/RoutePanel.vue";
import ViewFileEditor from "./components/views/ViewFileEditor.vue";

createApp(App)
    // .use(router)
    .component("view-files", ViewFiles)
    .component("view-home", ViewHome)
    .component("view-file-editor", ViewFileEditor)
    .component("dockview-vue", DockviewVue)
    .component("route-panel", RoutePanel)
    .mount("#app");
