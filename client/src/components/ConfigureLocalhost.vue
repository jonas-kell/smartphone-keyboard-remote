<template>
    <p>The key from backend:</p>
    <input type="text" readonly v-model="backendPSK" />
    <button @click="regenerateKey">Regenerate</button>

    <QRCodeDisplay :text="backendPSK"></QRCodeDisplay>

    Go to <a href="https://jonas-kell.github.io/smartphone-keyboard-remote/#/">hosted version</a> with mobile device
</template>

<script setup lang="ts">
    import QRCodeDisplay from "./QRCodeDisplay.vue";
    import useMainStore from "../stores/main";
    import { ref } from "vue";

    const mainStore = useMainStore();

    const backendPSK = ref("");
    async function getPSKFromBackend() {
        let [responseMethod, responsePayload] = await mainStore.communicateWithBackendUnencryptedLocalhost("get_psk", "");

        if (responseMethod == "ret_psk") {
            backendPSK.value = responsePayload;
        } else {
            console.error(responseMethod, responsePayload);
        }
    }
    getPSKFromBackend();

    async function regenerateKey() {
        backendPSK.value = "";
        let [responseMethod, responsePayload] = await mainStore.communicateWithBackendUnencryptedLocalhost("regenerate_psk", "");

        if (responseMethod == "ret_psk") {
            backendPSK.value = responsePayload;
        } else {
            console.error(responseMethod, responsePayload);
        }
    }
</script>

<style scoped></style>
