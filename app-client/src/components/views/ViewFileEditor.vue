<template>
    <div class="view-file-editor">
        <EditorContent :editor="editor" />
    </div>
</template>

<script lang="ts" setup>
import { onMounted, onUpdated, ref } from "vue";
import { core } from "@tauri-apps/api";
import { readFile } from "@tauri-apps/plugin-fs";
import { useEditor, EditorContent } from "@tiptap/vue-3"
import StarterKit from "@tiptap/starter-kit"

// The filepath property.
const props = defineProps<{
    path: string | null;
}>();

// The prose mirror editor instance.
const editor = useEditor({
    content: "",
    extensions: [StarterKit],
});

// The currently opened file path.
const currentFile = ref<string | null>(props.path);

// Load note content when component is mounted. Currently, there is
// no error handling and only one file saved.
onMounted(async () => {
    // Save note content after every update. Currently, there is
    // no error handling and only one file saved.
    editor.value!.on("update", async () => {
        const content = editor.value?.getHTML() || "";
        await core.invoke("save_note", { text: content });
    });
});

// Update the editor content when the file path changes. This save-closes
// the current file, updates path and reads new file.
onUpdated(async () => {
    // TODO save file
    
    if (props.path === currentFile.value) return;
    // TODO close file
    
    currentFile.value = props.path;
    if (!currentFile.value) {
        editor.value!.commands.setContent("");
        return;
    }

    let content: string;

    try {
        const file = await readFile(currentFile.value);
        console.debug(file);
        content = typeof file === "string" ? file : new TextDecoder().decode(file);
    } catch (e) {
        content = `<p><b>Error</b>: Could not read file at path ${currentFile.value}</p>`;
        // TODO show error to user
    }

    editor.value!.commands.setContent(content);
});
</script>

<style lang="scss" scoped>
@use "@/assets/main.scss" as *;

.view-file-editor {
    flex: 1;
    display: flex;
    padding: 10px;
    flex-direction: column;

    :deep(.ProseMirror) {
        flex: 1;
        width: 100%;
        min-height: 100%;
        overflow: auto;

        &:focus {
            outline: none;
        }
    }
}
</style>

