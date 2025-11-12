<template>
    <div class="view-directory">
        <ViewDirectoryContent />
        <ViewFileEditor />
    </div>
</template>

<script lang="ts" setup>
import ViewDirectoryContent from "./ViewDirectoryContent.vue";
import ViewFileEditor from "./ViewFileEditor.vue";
</script>

<style lang="scss" scoped>
@use "@/assets/main.scss" as *;

.view-directory {
    flex: 1;
    display: flex;
    flex-direction: row;
}
</style>

