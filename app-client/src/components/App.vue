<template>
    <div class="view-app">
        <TitleBar />
        <div class="view-app-content">
            <!-- <DockviewVue
            class="dockview-theme-light h-screen w-screen"
            :components="components"
            @ready="onReady"
            /> -->
            <!-- <dockview-vue
            style="width:100%;height:100%"
            class="dockview-theme-abyss"
            @ready="onReady"
            :locked=true></dockview-vue> -->
            <dockview-vue
                style="width:100%;height:100%"
                @ready="onReady"
            />
        </div>
    </div>
</template>

<script lang="ts" setup>
import { h } from "vue";
// import { RouterView, useRouter } from "vue-router";
import { type DockviewReadyEvent } from "dockview-vue";

import TitleBar from "./TitleBar.vue";
// import RoutePanel from "./dockview/RoutePanel.vue";

// const router = useRouter();

// Add panels when layout mounts
function onReady(event: DockviewReadyEvent) {
    // Home panel as split view (to the right of sidebar)
    event.api.addPanel({
        id: "home",
        component: "view-file-editor",
        tabComponent: "route-panel",
        title: "Home",
        // params: { path: "/" },
    });

    // Sidebar panel
    event.api.addPanel({
        id: "sidebar",
        component: "view-files",
        tabComponent: "route-panel",
        title: "Sidebar",
        // params: { path: "/files" },
        position: {
            direction: "left",
            referencePanel: "home"
        },
        maximumWidth: 240,
        minimumWidth: 0,
    });

    event.api.panels.forEach((panel) => {
        panel.group.header.hidden = true;
    });

    // // Optional: sync active panel with Vue Router
    // event.api.onDidActivePanelChange((event: any) => {
    //     const path = event.panel.params?.path
    //     if (path) {
    //         router.push(path)
    //     }
    // })
}
</script>

<style lang="scss" scoped>
@import "dockview-vue/dist/styles/dockview.css";
@import "@/assets/main.scss";

.view-app {
    display: flex;
    width: 100vw;
    height: 100vh;
    flex-direction: column;
    justify-content: flex-start;

    background-color: $primary-bg;

    .view-app-content {
        flex: 1;
        display: flex;
        width: 100%;
        height: 100%;
    }
}
</style>

<style>
body {
    font-family: Avenir, Helvetica, Arial, sans-serif;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    text-align: center;
    color: white;
    margin: 0;
    padding: 0;
}
</style>

<!--

import 'dockview-vue/dist/styles/dockview.css';
import { PropType, createApp, defineComponent } from 'vue';
import {
    DockviewVue,
    DockviewReadyEvent,
    IDockviewPanelProps,
} from 'dockview-vue';

const Panel = defineComponent({
    name: 'Panel',
    props: {
        params: {
            type: Object as PropType<IDockviewPanelProps>,
            required: true,
        },
    },
    data() {
        return {
            title: '',
        };
    },

    mounted() {
        const disposable = this.params.api.onDidTitleChange(() => {
            this.title = this.api.title;
        });
        this.title = this.api.title;

        return () => {
            disposable.dispose();
        };
    },

    template: `
      <div style="height:100%;padding:20px;">
        <span style="color:white;">{{ title }}</span>
      </div>`,
});

const App = defineComponent({
    name: 'App',
    components: {
        'dockview-vue': DockviewVue,
        default: Panel,
    },
    methods: {
        onReady(event: DockviewReadyEvent) {
            event.api.addPanel({
                id: 'panel_1',
                component: 'default',
            });
            event.api.addPanel({
                id: 'panel_2',
                component: 'default',
                position: {
                    direction: 'right',
                    referencePanel: 'panel_1',
                },
            });
            event.api.addPanel({
                id: 'panel_3',
                component: 'default',
                position: {
                    direction: 'below',
                    referencePanel: 'panel_1',
                },
            });
            event.api.addPanel({
                id: 'panel_4',
                component: 'default',
            });
            event.api.addPanel({
                id: 'panel_5',
                component: 'default',
            });
        },
    },
    template: `
      <dockview-vue
        style="width:100%;height:100%"
        class="dockview-theme-abyss"
        @ready="onReady"
        :locked=true
      </dockview-vue>`,
});

const app = createApp(App);
app.config.errorHandler = (err) => {
    console.log(err);
};
app.mount(document.getElementById('app')!);


-->