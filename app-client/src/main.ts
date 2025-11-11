import { createApp } from "vue";
// import { createWebHistory, createRouter } from "vue-router";

import App from "./components/App.vue";
// import routes from "./routes";

// const router = createRouter({
//     history: createWebHistory(),
//     routes,
// });

createApp(App)
    // .use(router)
    .mount("#app");
