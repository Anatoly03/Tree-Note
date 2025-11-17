<template>
    <div class="view-directory-content" :class="{'root-directory': props.isRoot}">
        <div class="directory-header" v-if="!props.isRoot" @click="toggleVisibility()">
            {{directoryName}}
            <span class="spoiler">
                <font-awesome-icon icon="fa-solid fa-chevron-down" v-if="isVisible" />
                <font-awesome-icon icon="fa-solid fa-chevron-right" v-if="!isVisible" />
            </span>
        </div>
        <div class="content" :class="{'hide': !isVisible}">
            <ViewDirectoryContent
                v-for="file in subdirectoryTree"
                :key="file.id"
                :currentDirectory="file.id"
                :selectedFile="props.selectedFile"
                @update:selectedFile="emit('update:selectedFile', $event)"
            />
            <ViewDirectoryEntryFile
                v-for="file in fileTree"
                :key="file.id"
                :fullPath="file.id"
                :selectedFile="props.selectedFile"
                @update:selectedFile="emit('update:selectedFile', $event)"
            />
        </div>
    </div>
</template>

<script lang="ts" setup>
import { onMounted, onUpdated, ref } from "vue";
import { useRouter } from "vue-router";
import { readDir } from "@tauri-apps/plugin-fs";
import ViewDirectoryEntryFile from "./ViewDirectoryEntryFile.vue";

const props = defineProps<{
    isRoot?: boolean;
    currentDirectory?: string;
    selectedFile?: string;
}>();

const emit = defineEmits<{
    (e: "update:selectedFile", value: string | null): void;
}>();

const router = useRouter();
const directory = props.currentDirectory || router.currentRoute.value.params.directory as string;
const directoryName = directory.split("/").pop() || "/";
const fileTree = ref<any[]>([]);
const subdirectoryTree = ref<any[]>([]);
const isVisible = ref(true);

onMounted(() => readDirectoryDeep());
// onUpdated(() => readDirectoryDeep());

async function readDirectoryDeep() {
    const contents = await readDir("/" + directory);

    fileTree.value = [];
    for (const item of contents) {
        if (item.name.startsWith(".")) continue;

        const fullPath = '/' + directory + "/" + item.name;
        const isMatch = props.selectedFile && fullPath === props.selectedFile;

        const name = item.name.lastIndexOf('.') > 0
            ? item.name.substring(0, item.name.lastIndexOf('.'))
            : item.name;
        const extension = item.name.lastIndexOf('.') > 0
            ? item.name.substring(item.name.lastIndexOf('.') + 1)
            : null;

        if (item.isDirectory) {
            subdirectoryTree.value.push({
                id: fullPath,
                name,
                isDir: item.isDirectory,
                isFile: item.isFile,
                isSelected: isMatch,
            });
            continue;
        }

        if (item.isFile) {
            // TODO make this dynamic
            if (extension !== "md") continue;

            fileTree.value.push({
                id: fullPath,
                name,
                extension,
                isDir: item.isDirectory,
                isFile: item.isFile,
                isSelected: isMatch,
            });
            continue;
        }

        // neither file, nor folder, ignore.
    }

    // Select first file by default.
    if (!props.selectedFile) {
        const firstFile = fileTree.value.find(f => f.isFile);
        if (firstFile) {
            emit("update:selectedFile", firstFile.id);
        }
    }

    // for (const file of fileTree.value) {
    //     file.isSelected = props.selectedFile === file.id;
    // }
}

function openFile(path: string) {
    const file = fileTree.value.find(f => f.id === path);
    if (!file || file.isDir) return;

    emit("update:selectedFile", path);
}

// Hides or opensthe directory contents
function toggleVisibility() {
    isVisible.value = !isVisible.value;
}
</script>

<style lang="scss" scoped>
@use "@/assets/main.scss" as *;

.view-directory-content {
    display: flex;
    padding: 0 0 0 10px;
    flex-direction: column;
    gap: 2px;
    border-radius: 4px;
    overflow: auto;

    &.root-directory {
        padding: 10px;
    }

    .directory-header {
        cursor: pointer;

        .spoiler {
            color: $fg-muted;
            font-size: 0.6em;
        }
    }

    &:has(> .directory-header:hover) {
        background-color: $bg-accent-light;
    }

    .content {
        overflow: hidden;
        max-height: 1200px; // large enough to contain typical content
        opacity: 1;
        transition: max-height 220ms cubic-bezier(.2,.8,.2,1), opacity 180ms ease;
        will-change: max-height, opacity;

        &.hide {
            max-height: 0;
            opacity: 0;
            padding-top: 0;
            padding-bottom: 0;
        }
    }

    // &:not(.root-directory) {
    //     .content {
    //         border-left: 1px solid $bg-accent-dark;
    //     }
    // }
}
</style>
