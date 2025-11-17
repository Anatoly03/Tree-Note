<template>
    <div class="view-home">
        <div class="app-header">
            <div class="app-logo">
                <img src="/icon.png" srcset="/icon.svg" width="65%" />
            </div>
            <div class="app-title">Tree Note</div>
        </div>
        <div class="quick-menu">
            <div class="menu-action" @click="selectDirectory()">Open Folder</div>
        </div>
    </div>
</template>

<script lang="ts" setup>
import { open } from '@tauri-apps/plugin-dialog';
import { useRouter } from 'vue-router';

const router = useRouter();

async function selectDirectory() {
    const directory = await open({
        multiple: false,
        directory: true
    });

    if (!directory) return;

    // TODO sanitize directory is not null
    // TODO test directory is not file

    router.push('/e' + directory);
}
</script>

<style lang="scss" scoped>
@use "sass:color";
@use "@/assets/main.scss" as *;

.view-home {
    display: flex;
    width: 100vw;
    height: 100vh;
    flex-direction: column;
    justify-content: flex-start;

    .app-header {
        display: flex;
        flex-direction: column;
        align-items: center;
        padding-bottom: 20px;
        background-color: $bg-accent;
        
        margin: 0 40px;
        padding-top: 10%;
        border-radius: 0 0 8px 8px;
        
        .app-logo {
            display: flex;
            margin: 10px auto;
            width: 128px;
            aspect-ratio: 1;
            border-radius: 20px;

            text-align: center;
            background-color: $bg-accent-dark;

            &:hover {
                background-color: #40bf71;
            }

            transition: background-color 0.3s ease;

            img {
                margin: auto;
                width: 65%;
            }
        }

        .app-title {
            font-size: 2.5rem;
            font-weight: bold;
            color: $fg-default;
        }
    }

    .quick-menu {
        display: flex;
        flex-direction: row;
        align-items: center;
        background-color: $bg-accent;
        
        margin: 20px 40px;
        border-radius: 8px;

        background-color: $bg-secondary;

        .menu-action {
            margin: 5px 10px;
            padding: 5px 10px;
            border-radius: 6px;
            // background-color: $bg-primary;
            color: $fg-default;
            font-weight: 500;
            cursor: pointer;

            &:hover {
                background-color: color.adjust($bg-secondary, $lightness: -10%)
            }

            transition: background-color 0.3s ease;
        }
    }
}
</style>
