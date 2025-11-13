<template>
    <div class="view-file-editor">
        <EditorContent :editor="editor" />
    </div>
</template>

<script lang="ts" setup>
import { core } from "@tauri-apps/api";
import { useEditor, EditorContent } from "@tiptap/vue-3"
import StarterKit from "@tiptap/starter-kit"
import { onMounted, onUpdated } from "vue";

// The filepath property.
const props = defineProps<{
    path: string | null;
}>();

// The prose mirror editor instance.
const editor = useEditor({
    content: "",
    extensions: [StarterKit],
});

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

onUpdated(async () => {
    // const response: string = await core.invoke("load_note");
    // editor.value!.commands.setContent(response);

    editor.value!.commands.setContent(`<p><b>Path</b>: ${props.path}</p>`);
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

