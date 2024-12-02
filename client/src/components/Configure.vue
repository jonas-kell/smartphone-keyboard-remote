<template>
    <router-link to="/"><h1>Configure Mobile Device</h1></router-link>

    <h2>Install Server on your Machine in LAN</h2>
    <p>
        <a href="https://github.com/jonas-kell/smartphone-keyboard-remote?tab=readme-ov-file#run-the-backend">HERE</a>
        are installation instructions.
    </p>

    <h2>Set IP</h2>
    <p>The IP will get printed under <b>LAN access from</b> when the server starts</p>
    <input type="text" v-model="mainStore.ip" placeholder="xxx.xxx.xxx.xxx" />

    <h2>Exchange PSK</h2>
    <p>We need to obtain a PSK from the server.</p>
    <p>If this is unset, then we can paste it into the textfield or scan the QR-Code the server shows with our mobile device:</p>
    <h2>
        <span v-if="mainStore.serverConnectionEstablished" style="color: green">Connection Established</span>
        <span v-else style="color: red">Connection Not Established</span>
    </h2>
    <input
        type="password"
        v-model="mainStore.psk"
        @blur="
            (res) => {
                const inputVal = (res.target as any).value;
                mainStore.tryValidateConnection(inputVal);
            }
        "
    />

    <QRCodeReader
        v-if="!mainStore.psk"
        @scanned="
            (res) => {
                mainStore.tryValidateConnection(res);
            }
        "
    ></QRCodeReader>
</template>

<script setup lang="ts">
    import QRCodeReader from "./QRCodeReader.vue";
    import useMainStore from "../stores/main";

    const mainStore = useMainStore();
</script>

<style scoped></style>
