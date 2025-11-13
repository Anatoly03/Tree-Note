<template>
    <div class="view-app">
        <TitleBar />
        <router-view />
    </div>
</template>

<script lang="ts" setup>
import TitleBar from "./views/TitleBar.vue";
import ViewDirectory from "./views/ViewDirectory.vue";
import ViewHome from "./views/ViewHome.vue";

defineOptions({
    routes: [
        { path: "/", component: ViewHome },
        { path: "/e/:directory(.*)", component: ViewDirectory },
    ]
})
</script>

<style lang="scss" scoped>
@use "@/assets/main.scss" as *;

.view-app {
    display: flex;
    height: 100%;
    flex-direction: column;

    background-color: $bg-default;
}
</style>
