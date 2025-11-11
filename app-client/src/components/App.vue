<template>
    <div class="view-app">
        <ViewHome />
    </div>
</template>

<script lang="ts" setup>
import ViewHome from "./views/ViewHome.vue";
</script>

<style lang="scss" scoped>
@import "@/assets/main.scss";

.view-app {
    display: flex;
    width: 100vw;
    height: 100vh;
    flex-direction: column;
    justify-content: flex-start;

    background-color: $bg-default;

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
    margin: 0;
    padding: 0;
}
</style>
