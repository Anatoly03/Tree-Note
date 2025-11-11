
import ViewHome from "./components/views/ViewHome.vue";
import ViewFiles from "./components/views/ViewFiles.vue";
import { DockviewVue } from "dockview-vue";

export default [
    {
        path: "/",
        component: DockviewVue,
        children: [
            { path: "", component: ViewHome },
            { path: "files", component: ViewFiles },
        ],
    },
];
