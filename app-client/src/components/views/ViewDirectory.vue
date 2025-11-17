<template>
    <div class="view-directory">
        <ViewSideMenu />
        <div class="directory-content">
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

    .directory-content {
        display: flex;
        flex: 0.3;
        flex-direction: column;
        background-color: $bg-secondary;

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

