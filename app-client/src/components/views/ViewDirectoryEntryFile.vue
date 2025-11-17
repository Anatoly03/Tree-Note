<template>
    <div class="view-directory-entry-file" @click.right.prevent="startFileRename()">
        <input
            ref="filenameInput"
            v-model="newName"
            type="text"
            class="inline"
            :class="{'hide':!isRenaming}"
            @keydown.enter.prevent="renameFile()"
            @blur="renameFile()"
        />
        <div class="filename" v-if="!isRenaming" @click.left="openFile()">{{ name }}</div>
    </div>
</template>

<script lang="ts" setup>
import { ref, nextTick } from "vue";
import { rename } from "@tauri-apps/plugin-fs";

const props = defineProps<{
    selectedFile?: string;
    fullPath: string;
}>();

const filenameInput = ref<HTMLInputElement | null>(null);
const isRenaming = ref(false);

const fileName = props.fullPath.split("/").pop() || props.fullPath;

const baseDirectory = props.fullPath.lastIndexOf('/') > 0
    ? props.fullPath.substring(0, props.fullPath.lastIndexOf('/'))
    : "/";
const name = ref(fileName.lastIndexOf('.') > 0
    ? fileName.substring(0, fileName.lastIndexOf('.'))
    : fileName);
const extension = fileName.lastIndexOf('.') > 0
    ? fileName.substring(fileName.lastIndexOf('.') + 1)
    : '';
const newName = ref(name.value);

const emit = defineEmits<{
    (e: "update:selectedFile", value: string | null): void;
}>();

// Sends update to parent component to set this file as selected.
function openFile() {
    emit("update:selectedFile", props.fullPath);
}

// Enables renaming mode.
async function startFileRename() {
    isRenaming.value = true;
    await nextTick();
    filenameInput.value?.focus();
    // select the current text for easier renaming
    filenameInput.value?.select();
}

// Enables renaming mode.
async function renameFile() {
    isRenaming.value = false;
    filenameInput.value?.blur();

    if (newName.value.trim() === "") {
        newName.value = name.value;
        isRenaming.value = false;
        return;
    }

    // const previousName = extension ? `${name.value}.${extension}` : name.value;
    const finalName = extension ? `${newName.value.trim()}.${extension}` : newName.value.trim();
    const newFullPath = baseDirectory === "/"
        ? `/${finalName}`
        : `${baseDirectory}/${finalName}`;

    try {
        // TODO BUG: fix: rename throws error despite successful move
        await rename(props.fullPath, newFullPath);
        name.value = newName.value.trim();
    } catch (e) {
        console.error('could not rename file:', e);
        // TODO show error to user
        newName.value = name.value;
    }
}
</script>

<style lang="scss" scoped>
@use "@/assets/main.scss" as *;
@use "@/assets/ui.scss" as *;

.view-directory-entry-file {
    display: flex;
    border-radius: 4px;

    cursor: pointer;
    transition: background-color 0.2s ease;

    .filename {
        flex: 1;
        padding: 1px 10px;
    }

    input[type="text"].inline {
        flex: 1;
        padding: 1px 10px;
    }

    & > .hide {
        display: none;
    }

    &:hover, &.content-selected {
        background-color: $bg-accent-light;
    }

    :deep(svg) {
        color: $fg-accent;
    }
}
</style>

