<template>
    <div class="view-directory">
        <ViewSideMenu />
        <ViewDirectoryContent :selectedFile="selectedFile" @update:selectedFile="openFile" />
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
}
</style>

