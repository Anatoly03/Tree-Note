<template>
    <div class="view-files">
        <ul>
            <li v-for="file in files" :key="file.name">
                <a :href="file.route">{{ file.name }}</a> ({{ file.fileType }})
            </li>
        </ul>
    </div>
</template>

<script lang="ts" setup>
import { reactive } from "vue";

const files = reactive([
    { name: "Test1", route: "test1.txt", fileType: 'text' },
    { name: "Test2", route: "test2.md", fileType: 'markdown' },
    { name: "Test3", route: "test3.txt", fileType: 'text' },
]);
</script>

<style lang="scss" scoped>
@import "@/assets/main.scss";
@import "@/assets/ui.scss";

.view-files {
    width: 100%;
    min-height: 100%;
    background-color: $primary-bg;
    padding: 16px;
}
</style>
