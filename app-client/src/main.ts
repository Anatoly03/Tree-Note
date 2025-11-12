import { createApp } from "vue";
import { createWebHistory, createRouter } from "vue-router";
import { library } from '@fortawesome/fontawesome-svg-core';
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome';

import { fas } from '@fortawesome/free-solid-svg-icons';
import { far } from '@fortawesome/free-regular-svg-icons';

import App from "./components/App.vue";

library.add(fas, far);

const router = createRouter({
    history: createWebHistory(),
    routes: App.routes,
});

createApp(App)
    .component('font-awesome-icon', FontAwesomeIcon)
    .use(router)
    .mount("#app");
