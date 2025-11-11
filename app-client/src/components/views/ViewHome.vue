<template>
    <div class="view-home">
        <div class="input-group">
            <input type="text" v-model="name" />
            <button type="submit" @click="say_hello">say hello</button>
        </div>
        <p>{{ greeting }}</p>
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
@import "@/assets/main.scss";
@import "@/assets/ui.scss";

.view-home {
    width: 100%;
    min-height: 100%;
    background-color: $primary-bg;
    padding: 16px;
}
</style>
