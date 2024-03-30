<script setup lang="ts">
import { computed, ref } from "vue";
import DeviceInfo from "../interface/DeviceInfo";
import { useDeviceStateStore } from "@/stores/DeviceStateStore";
import DoneIcon from "@/assets/icons/DoneIcon.vue";
import SendIcon from "@/assets/icons/SendIcon.vue";
import { useToast } from "vue-toast-notification";

const props = defineProps<{
    device: DeviceInfo;
}>();

const toast = useToast();

const deviceStateStore = useDeviceStateStore();

const deviceConnected = computed(() => {
    return (
        deviceStateStore.connectedDevice?.adb_serial === props.device.adb_serial
    );
});

const deviceConnecting = ref(false);

function connectToDevice() {
    deviceConnecting.value = true;
    deviceStateStore
        .connectTo(props.device)
        .catch((e) => {
            console.error(e);
            toast.error("Failed to connect to device");
        })
        .finally(() => {
            deviceConnecting.value = false;
        });
}
</script>

<template>
    <div class="px-4 flex justify-between items-center">
        <div class="flex flex-col">
            <p class="truncate max-w-56">{{ props.device.name }}</p>
            <p class="text-gray-500 text-xs -mt-2">
                {{ props.device.adb_serial }}
            </p>
        </div>
        <md-outlined-icon-button
            :disabled="deviceConnected"
            @click="connectToDevice"
        >
            <DoneIcon color="#55EE55" v-if="deviceConnected" />
            <SendIcon v-else-if="!deviceConnecting" />
            <md-circular-progress indeterminate v-if="deviceConnecting" />
        </md-outlined-icon-button>
    </div>
</template>
