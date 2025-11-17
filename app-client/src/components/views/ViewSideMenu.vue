<template>
    <div class="view-side-menu">
        <div class="section">
            <div class="side-menu-button" @click="toggleSidebar()" :class="{'selected': sidebarToggle}">
                <font-awesome-icon icon="fa-regular fa-folder-open" />
            </div>
            <div class="side-menu-button disabled">
                <font-awesome-icon icon="fa-solid fa-diagram-project" />
            </div>
        </div>
        <div class="section">
            <div class="side-menu-button disabled">
                <font-awesome-icon icon="fa-solid fa-gear" />
            </div>
            <div class="side-menu-button">
                <!-- TODO create new window, keep this directory opened -->
                <router-link to="/">
                    <font-awesome-icon icon="fa-solid fa-house-chimney-user" />
                </router-link>
            </div>
        </div>
    </div>
</template>

<script lang="ts" setup>
import { ref } from "vue";

const sidebarToggle = ref(true);

const emit = defineEmits<{
    (e: "toggleSidebar", value: boolean): void;
}>();

function toggleSidebar() {
    sidebarToggle.value = !sidebarToggle.value;
    emit("toggleSidebar", sidebarToggle.value);
}
</script>

<style lang="scss" scoped>
@use "@/assets/main.scss" as *;

.view-side-menu {
    display: flex;
    flex-direction: column;
    justify-content: space-between;

    background-color: $bg-default;
    
    .section {
        display: flex;
        flex-direction: column;
        align-items: flex-start;

        margin: 8px;
        width: 36px;
        gap: 6px;
    }

    .side-menu-button {
        display: flex;
        width: 100%;
        aspect-ratio: 1;
        align-items: center;
        justify-content: center;
        border-radius: 4px;
        
        color: $fg-accent;
        cursor: pointer;

        :deep(a) {
            color: $fg-accent;
        }

        &.disabled {
            cursor: not-allowed;
            color: $fg-muted;
        }

        &:not(.disabled):hover {
            background-color: $bg-accent;
        }

        &.selected {
            background-color: $bg-accent-light;

            &:not(.disabled) {
                cursor: default;
                
                &:hover {
                    background-color: $bg-accent;
                }
            }
        }
    }
}
</style>

