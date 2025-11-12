<template>
    <div class="view-directory-content">
        <v-for :key="file.id" v-for="file in fileTree">
            <div class="content">
                {{ file.name }}
            </div>
        </v-for>

        <!-- TODO remove -->
        <router-link to="/">Back to Home</router-link>
    </div>
</template>

<script lang="ts" setup>
import { onMounted, ref } from 'vue';
import { useRouter } from 'vue-router';
import { readDir } from '@tauri-apps/plugin-fs';

const router = useRouter();
const fileTree = ref<any[]>([]);

onMounted(async () => {
    const directory = router.currentRoute.value.params.directory as string;
    const contents = await readDir('/' + directory);
    console.log(contents);

    fileTree.value = contents;
});
</script>

<style lang="scss" scoped>
@use "@/assets/main.scss" as *;

.view-directory-content {
    flex: 0.3;
    display: flex;
    padding: 10px;
    flex-direction: column;
    gap: 2px;

    background-color: $bg-secondary;

    .content {
        padding: 2px;
        border-radius: 4px;
        
        cursor: pointer;
        transition: background-color 0.2s ease;

        &:hover {
            background-color: $bg-accent-light;
        }
    }
}
</style>

