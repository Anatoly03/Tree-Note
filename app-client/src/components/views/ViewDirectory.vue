<template>
    <div class="view-directory">
        <ViewSideMenu @toggleSidebar="sidebarToggle = $event" />
        <div class="directory-content" :class="{'hide': !sidebarToggle}">
            <div class="top-menu">
                <a>
                    <font-awesome-icon icon="fa-solid fa-file-circle-plus" />
                </a>
                <a>
                    <font-awesome-icon icon="fa-solid fa-folder-plus" />
                </a>
            </div>
            <ViewDirectoryContent
                isRoot
                :selectedFile="selectedFile"
                @update:selectedFile="openFile"
            />
        </div>
        <ViewFileEditor :path="selectedFile" />
    </div>
</template>

<script lang="ts" setup>
import { ref } from "vue";
import ViewSideMenu from "./ViewSideMenu.vue";
import ViewDirectoryContent from "./ViewDirectoryContent.vue";
import ViewFileEditor from "./ViewFileEditor.vue";

const sidebarToggle = ref(true);
const selectedFile = ref<string | null>(null);

// Update the selected file when a file is opened.
function openFile(value: string | null) {
    selectedFile.value = value;
}
</script>

<style lang="scss" scoped>
@use "@/assets/main.scss" as *;

.view-directory {
    flex: 1;
    display: flex;
    flex-direction: row;
    overflow: hidden;

    .directory-content {
        display: flex;
        flex: 0.3;
        flex-direction: column;
        // background-color: $bg-secondary;

        border-right: 1px solid $fg-light-secondary;
        
        overflow: hidden;
        opacity: 1;
        transition: flex 220ms cubic-bezier(.2,.8,.2,1), opacity 180ms ease;
        will-change: flex, opacity;

        &.hide {
            flex: 0;
            opacity: 0;
            padding-top: 0;
            padding-bottom: 0;
        }

        .top-menu {
            display: flex;
            justify-content: center;
            flex-direction: row;
            padding: 8px 8px 0 8px;
            gap: 4px;

            a {
                padding: 8px;
                border-radius: 4px;

                &:hover {
                    background-color: $bg-accent-light;
                }

                text-align: center;
                cursor: pointer;
            }
        }
    }
}
</style>

