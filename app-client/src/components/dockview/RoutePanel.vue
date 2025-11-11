<script lang="ts" setup>
import { IDockviewPanelProps } from "dockview-vue";
import { defineProps, onMounted, PropType, ref } from "vue";

const title = ref('');

const props = defineProps({
    params: {
        type: Object as PropType<IDockviewPanelProps>,
        required: true,
    },
});

onMounted(() => {
    const disposable = props.params.api.onDidTitleChange(() => {
        title.value = props.params.api.title ?? 'undefined';
    });
    title.value = props.params.api.title ?? 'undefined';

    return () => disposable.dispose();
});
</script>

<template>
    <div class="view-panel">
        <span style="color: white">{{ title }}</span>
    </div>
</template>

<style lang="scss" scoped>
.view-panel {
    height: 100%;
    align-self: stretch;
}
</style>
