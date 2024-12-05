<template>
    <router-link to="/"><h1>Configure Mobile Device</h1></router-link>

    <h2>Install Server on your Machine in LAN</h2>
    <p>
        <a href="https://github.com/jonas-kell/smartphone-keyboard-remote?tab=readme-ov-file#run-the-backend">HERE</a>
        are installation instructions.
    </p>

    <h2>Mixed-Content-Settings:</h2>
    <p>
        Read the explanation for this setting
        <a href="https://github.com/jonas-kell/smartphone-keyboard-remote?tab=readme-ov-file#hosted-version">HERE</a>. Change the
        Chrome Setting (Or find out how to change in another browser).
    </p>
    <p>
        Add "http://xxx.xxx.xxx.xxx:5173/" (with ip replaced by what server shows) to the list and save.
        <a href="chrome://flags/#unsafely-treat-insecure-origin-as-secure">
            chrome://flags/#unsafely-treat-insecure-origin-as-secure
        </a>
    </p>

    <h2>Backend Connection</h2>
    <p>Preferably: Scan the QR-Code the server shows with our mobile device:</p>
    <h2>
        <span v-if="mainStore.serverConnectionEstablished" style="color: green">Connection Established</span>
        <span v-else style="color: red">Connection Not Established</span>
    </h2>

    <QRCodeReader
        v-if="!mainStore.psk"
        @scanned="
            (res) => {
                const parts = res.split(':', 2);
                const ip = parts[0] ?? null;
                const psk = parts[1] ?? null;
                mainStore.tryValidateConnection(ip, psk);
            }
        "
    ></QRCodeReader>

    <h2>Values (Manual)</h2>
    <button
        @click="
            {
                mainStore.ip = null;
                mainStore.psk = null;
            }
        "
    >
        Clear Config
    </button>

    <h3>Exchange IP</h3>
    <p>The IP will get printed under <b>LAN access from</b> when the server starts</p>
    <p>If this is unset, then you can paste it into the textfield</p>
    <input
        type="text"
        v-model="mainStore.ip"
        placeholder="xxx.xxx.xxx.xxx"
        @blur="
            (res) => {
                const inputVal = (res.target as any).value;
                mainStore.tryValidateConnection(inputVal, null);
            }
        "
    />

    <h3>Exchange PSK</h3>
    <p>We need to obtain a PSK from the server.</p>
    <p>If this is unset, then you can paste it into the textfield</p>
    <input
        type="password"
        v-model="mainStore.psk"
        @blur="
            (res) => {
                const inputVal = (res.target as any).value;
                mainStore.tryValidateConnection(null, inputVal);
            }
        "
    />
</template>

<script setup lang="ts">
    import QRCodeReader from "./QRCodeReader.vue";
    import useMainStore from "../stores/main";

    const mainStore = useMainStore();
</script>

<style scoped></style>
