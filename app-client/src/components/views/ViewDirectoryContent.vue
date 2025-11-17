<template>
    <div class="view-directory-content">
        <div
            v-for="file in fileTree"
            :key="file.id"
            class="content"
            :class="{
                'content-dir': file.isDir,
                'content-file': file.isFile,
                'content-selected': file.isSelected,
            }"
            @click="openFile(file.id)"
        >
            <font-awesome-icon icon="fa-regular fa-folder" v-if="file.isDir" />
            <font-awesome-icon icon="fa-regular fa-file" v-if="file.isFile" />
            {{ file.name }}
        </div>

        <!-- TODO remove -->
        <div class="footer">
            <router-link to="/">Back to Home</router-link>
        </div>
    </div>
</template>

<script lang="ts" setup>
import { onMounted, onUpdated, ref } from "vue";
import { useRouter } from "vue-router";
import { readDir } from "@tauri-apps/plugin-fs";

const props = defineProps<{
    selectedFile: string | null;
}>();

const emit = defineEmits<{
    (e: "update:selectedFile", value: string | null): void;
}>();

const router = useRouter();
const fileTree = ref<any[]>([]);

onMounted(async () => {
    const directory = router.currentRoute.value.params.directory as string;
    const contents = await readDir("/" + directory);

    fileTree.value = [];
    for (const item of contents) {
        if (item.name.startsWith(".")) continue;

        const fullPath = '/' + directory + "/" + item.name;
        const isMatch = props.selectedFile && fullPath === props.selectedFile;

        fileTree.value.push({
            id: fullPath,
            name: item.name,
            isDir: item.isDirectory,
            isFile: item.isFile,
            isSelected: isMatch,
        });
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
    flex: 0.3;
    display: flex;
    padding: 10px;
    flex-direction: column;
    gap: 2px;

    background-color: $bg-secondary;

    .content {
        padding: 2px;
        border-radius: 4px;

        cursor: pointer;
        transition: background-color 0.2s ease;

        &:hover, &.content-selected {
            background-color: $bg-accent-light;
        }

        // TODO add folder traversal
        &.content-dir { cursor: not-allowed; }

        :deep(svg) {
            color: $fg-accent;
        }
    }

    .footer {
        margin-top: auto;
        padding-top: 10px;
        border-top: 1px solid $border-color;

        :deep(a) {
            color: $fg-accent;
            text-decoration: none;

            &:hover {
                text-decoration: underline;
            }
        }
    }
}
</style>
