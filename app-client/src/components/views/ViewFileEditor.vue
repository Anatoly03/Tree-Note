<template>
    <div class="view-file-editor">
        <div class="editor" contenteditable="true">
            <p>Hello World!</p>
        </div>
    </div>
</template>

<script lang="ts" setup>
import { core } from "@tauri-apps/api";
import { ref } from "vue";

const greeting = ref('');
const name = ref('');

async function say_hello() {
    const response: string = await core.invoke("greet", { name: name.value });
    greeting.value = response;
}
</script>

<style lang="scss" scoped>
@import "@/assets/ui.scss";

.view-file-editor {
    display: flex;
    flex-direction: column;
    overflow-y: scroll;
    width: 100%;
    height: 100%;
    background-color: $primary-bg;

    .editor {
        height: 100%;
        text-align: left;
        color: $primary-text;
        // padding: 16px;

        p {
            margin: 5px;
        }
    }
}
</style>
