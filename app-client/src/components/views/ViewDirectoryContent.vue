<template>
    <div class="view-directory-content">
        <div class="directory-header" v-if="!props.isRoot">
            {{directoryName}}
        </div>
        <ViewDirectoryContent
            v-for="file in subdirectoryTree"
            :key="file.id"
            :currentDirectory="file.id"
            :selectedFile="props.selectedFile"
            @update:selectedFile="emit('update:selectedFile', $event)"
        />
        <div
            v-for="file in fileTree"
            :key="file.id"
            @click="openFile(file.id)"
            class="file-entry"
        >
            {{ file.name }}
        </div>
    </div>
</template>

<script lang="ts" setup>
import { onMounted, onUpdated, ref } from "vue";
import { useRouter } from "vue-router";
import { readDir } from "@tauri-apps/plugin-fs";

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

onMounted(async () => {
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
});

onUpdated(() => {
    for (const file of fileTree.value) {
        file.isSelected = props.selectedFile === file.id;
    }
});

function openFile(path: string) {
    const file = fileTree.value.find(f => f.id === path);
    if (!file || file.isDir) return;

    emit("update:selectedFile", path);
}
</script>

<style lang="scss" scoped>
@use "@/assets/main.scss" as *;

.view-directory-content {
    display: flex;
    padding: 0 5px;
    flex-direction: column;
    gap: 2px;

    .file-entry {
        padding: 6px 10px;
        border-radius: 4px;

        cursor: pointer;
        transition: background-color 0.2s ease;

        &:hover, &.content-selected {
            background-color: $bg-accent-light;
        }

        :deep(svg) {
            color: $fg-accent;
        }
    }
}
</style>
