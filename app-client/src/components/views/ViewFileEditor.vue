<template>
    <div class="view-file-editor">
        <EditorContent :editor="editor" />
    </div>
</template>

<script lang="ts" setup>
import { useEditor, EditorContent } from "@tiptap/vue-3"
import StarterKit from "@tiptap/starter-kit"

const editor = useEditor({
    content: "<p>I'm running Tiptap with Vue.js.</p>",
    extensions: [StarterKit],
})
</script>

<style lang="scss" scoped>
@import "@/assets/main.scss";

.view-file-editor {
    flex: 1;
    display: flex;
    width: 100%;
    height: 100%;
    flex-direction: column;

    ::v-deep .ProseMirror {
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

