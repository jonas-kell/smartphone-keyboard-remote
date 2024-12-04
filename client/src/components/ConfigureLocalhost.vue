<template>
    <p>The key from backend:</p>
    <input type="text" readonly v-model="backendPSK" />
    <button @click="regenerateKey">Regenerate</button>

    <QRCodeDisplay :text="backendPSK"></QRCodeDisplay>

    Go to <a href="https://jonas-kell.github.io/smartphone-keyboard-remote/#/">hosted version</a> with mobile device
    <br />
    <br />
    <br />
    <button @click="shutdown">Shut down server</button>
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

    async function shutdown() {
        // it wil self-destruct non gracefully, no bother checking return of this message, sry
        mainStore.communicateWithBackendUnencryptedLocalhost("shutdown_server", "");

        // reload window to show server gone
        setTimeout(() => {
            location.reload();
        }, 500);
    }
</script>

<style scoped></style>
