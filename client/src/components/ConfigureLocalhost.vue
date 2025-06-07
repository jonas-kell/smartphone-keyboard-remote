<template>
    <p>The server Local Ip:</p>
    <input type="text" readonly v-model="backendLocalIp" />
    <button @click="copyToClipboard(backendLocalIp)">Copy Ip</button>
    <p>The server Pre-Shared-Key:</p>
    <input type="text" readonly v-model="backendPSK" />
    <button @click="copyToClipboard(backendPSK)">Copy PSK</button>
    <br />
    <br />
    <button @click="regenerateKey">Regenerate Key</button>

    <p>Go to <a href="https://jonas-kell.github.io/smartphone-keyboard-remote/#/">hosted version</a> with mobile device</p>
    <p>And scan with device to get full config</p>
    <QRCodeDisplay :text="backendLocalIp + ':' + backendPSK"></QRCodeDisplay>

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
    const backendLocalIp = ref("");

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

    async function getIPFromBackend() {
        backendPSK.value = "";
        let [responseMethod, responsePayload] = await mainStore.communicateWithBackendUnencryptedLocalhost("get_ip", "");

        if (responseMethod == "ret_ip") {
            backendLocalIp.value = responsePayload;
        } else {
            console.error(responseMethod, responsePayload);
        }
    }
    getIPFromBackend();

    function copyToClipboard(txt: string) {
        navigator.clipboard.writeText(txt);
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
