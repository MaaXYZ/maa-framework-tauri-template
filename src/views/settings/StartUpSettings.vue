<script setup lang="ts">
import { ref, watch } from "vue";
import { ClientType, allClientTypes } from "@/interface/StartUpConfig";
import { useMaaStateStore } from "@/stores/MaaStateStore";
import MdcSelect from "@/components/mdc/select/MdcSelect.vue";
import { snackbar } from "mdui/functions/snackbar";

const maaStateStore = useMaaStateStore();

const clientTypeOptions = allClientTypes.map((cType) => {
    return {
        label: cType,
        value: cType,
    };
});

const clientType = ref<ClientType>(
    maaStateStore.config?.startUp.clientType ?? "Official"
);

function setClientType(v: ClientType) {
    maaStateStore.setClientType(v).catch((error) => {
        snackbar({
            message: error.message,
        });
    });
}

watch(clientType, (v) => {
    setClientType(v);
});
</script>

<template>
    <div>
        <div class="settings_item flex flex-row justify-between">
            <p>Client Type</p>
            <MdcSelect
                v-model="clientType"
                :options="clientTypeOptions"
                variant="outlined"
            ></MdcSelect>
        </div>
    </div>
</template>
