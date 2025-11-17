<template>
    <div class="view-file-editor" @click="editor.commands.focus()">
        <EditorContent :editor="editor" />
    </div>
</template>

<script lang="ts" setup>
import { onMounted, onUpdated, ref } from "vue";
import { readFile, writeFile } from "@tauri-apps/plugin-fs";
import { useEditor, EditorContent } from "@tiptap/vue-3"
import StarterKit from "@tiptap/starter-kit"
import showdown from 'showdown';

// The filepath property.
const props = defineProps<{
    path: string | null;
}>();

// The markdown to HTML converter.
const converter = new showdown.Converter();

// The prose mirror editor instance.
const editor = useEditor({
    content: "",
    extensions: [StarterKit],
});

// The currently opened file path.
const currentFile = ref<string | null>(props.path);

// Save the current file content.
async function saveFile() {
    const html = editor.value?.getHTML() ?? '';
    const md = converter.makeMd(html);

    // TODO if currentFile is not set, save as untitled file
    if (!currentFile.value) return;

    try {
        const stream = new TextEncoder().encode(md);

        await writeFile(currentFile.value, stream);
    } catch (e) {
        console.error('could not save file:', e);
        // TODO show error to user
    }
}

// Load note content when component is mounted. Currently, there is
// no error handling and only one file saved.
onMounted(async () => {
    // Save note content after every update. Currently, there is
    // no error handling and only one file saved.
    editor.value!.on("update", () => saveFile());
});

// Update the editor content when the file path changes. This save-closes
// the current file, updates path and reads new file.
onUpdated(async () => {
    // saveFile();
    
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
        const text = typeof file === "string" ? file : new TextDecoder().decode(file);

        content = converter.makeHtml(text);
        editor.value!.commands.setContent(content);
    } catch (e) {
        // content = `<p><b>Error</b>: Could not read file at path ${currentFile.value}</p>`;
        // TODO show error to user
        console.error('could not read file:', e);
    }

});
</script>

<style lang="scss" scoped>
@use "@/assets/main.scss" as *;

.view-file-editor {
    flex: 1;
    display: flex;
    padding: 10px;
    flex-direction: column;
    overflow: auto;

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
