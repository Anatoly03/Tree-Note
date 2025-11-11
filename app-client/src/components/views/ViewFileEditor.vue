<template>
    <div class="view-file-editor">
        <EditorContent :editor="editor" />
    </div>
</template>

<script lang="ts" setup>
import { core } from "@tauri-apps/api";
import { useEditor, EditorContent } from "@tiptap/vue-3"
import StarterKit from "@tiptap/starter-kit"
import { onMounted } from "vue";

// The prose mirror editor instance.
const editor = useEditor({
    content: "",
    extensions: [StarterKit],
});

// Load note content when component is mounted. Currently, there is
// no error handling and only one file saved.
onMounted(async () => {
    const response: string = await core.invoke("load_note");
    editor.value!.commands.setContent(response);

    // Save note content after every update. Currently, there is
    // no error handling and only one file saved.
    editor.value!.on("update", async () => {
        const content = editor.value?.getHTML() || "";
        await core.invoke("save_note", { text: content });
    });
});
</script>

<style lang="scss" scoped>
@import "@/assets/main.scss";

.view-file-editor {
    flex: 1;
    display: flex;
    width: 100%;
    height: 100%;
    flex-direction: column;

    :deep(.ProseMirror) {
        // TODO remove
        background-color: $bg-secondary;

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

