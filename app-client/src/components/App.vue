<template>
    <div class="view-app">
        <TitleBar />
        <ViewFileEditor />
    </div>
</template>

<script lang="ts" setup>
import TitleBar from "./views/TitleBar.vue";
// import ViewHome from "./views/ViewHome.vue";
import ViewFileEditor from "./views/ViewFileEditor.vue";
</script>

<style lang="scss" scoped>
@import "@/assets/main.scss";

.view-app {
    display: flex;
    width: 100%;
    height: 100%;
    flex-direction: column;

    background-color: $bg-default;
}
</style>

<style>
body {
    font-family: Avenir, Helvetica, Arial, sans-serif;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    width: 100%;
    min-height: 100vh;
    margin: 0;
    padding: 0;
}
</style>
