<!-- TODO: redesign for non-mac software -->
<!-- TODO: redesign for mac fullscreen -->

<template>
    <div data-tauri-drag-region class="view-title-bar">
        <div class="section traffic-light"></div>
        <div class="section title">Title</div>
        <div class="section controls"></div>
    </div>
</template>

<style lang="scss" scoped>
@import "@/assets/main.scss";

.view-title-bar {
    display: flex;
    align-items: center;
    flex-direction: row;
    justify-content: space-between;

    background-color: $bg-accent-dark;
    border-bottom: $border;
    height: 36px;
    font-size: 18px;
    font-weight: bold;

    .section {
        flex: 1;

        &.traffic-light {
            min-width: 81px;
        }

        &.title {
            flex: 0;
            color: $fg-light-secondary;
            text-align: center;
            font-weight: 500;
            font-size: 0.95em;
        }
    }
}
</style>