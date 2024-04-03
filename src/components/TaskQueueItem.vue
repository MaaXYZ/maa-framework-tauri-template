<script setup lang="ts">
import TaskStatus from "@/interface/TaskStatus";
import { useTaskQueueStore } from "@/stores/TaskQueueStore";

const taskQueueStore = useTaskQueueStore();

const props = defineProps<{
    index: number;
    task: TaskStatus;
}>();

function removeCurrent() {
    taskQueueStore.removeFromQueue(props.index);
}

</script>

<template>
    <div ref="outer" class="item mx-1 text-center items-center shadow relative">
        <mdui-dropdown trigger="contextmenu" placement="bottom-end">
            <mdui-card
                clickable
                slot="trigger"
                class="flex flex-col w-full h-full layer"
            >
                <p class="text-center">{{ props.task.taskType }}</p>
                <mdui-linear-progress
                    v-if="props.task.state === 'Running'"
                ></mdui-linear-progress>
            </mdui-card>
            <mdui-menu>
                <mdui-menu-item @click="removeCurrent">
                    Remove
                </mdui-menu-item>
            </mdui-menu>
        </mdui-dropdown>
    </div>
</template>

<style scope>
.item {
    min-width: 150px;
    width: 150px;
    height: 60px;
    border-radius: 0.5rem;
    background-color: var(--md-ref-palette-neutral95);
}
</style>
