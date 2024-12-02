<template>
    <h1>Smartphone Keyboard Remote</h1>

    <span v-if="mainStore.isLocalhost">You are localhost</span>
    <span v-if="mainStore.isExternal">You are external</span>
    <span v-if="!mainStore.isLocalhost && !mainStore.isExternal">Page status pending</span>

    <h2>Generate</h2>

    <input type="text" v-model="generatorText" />
    <QRCodeDisplay :text="generatorText"></QRCodeDisplay>

    <h2>Reader</h2>
    <QRCodeReader
        @scanned="
            (res) => {
                outputText = res;
            }
        "
    ></QRCodeReader>
    <input type="text" v-model="outputText" />
</template>

<script setup lang="ts">
    import { ref } from "vue";
    import QRCodeDisplay from "./QRCodeDisplay.vue";
    import QRCodeReader from "./QRCodeReader.vue";
    import useMainStore from "./../stores/main";
    import { decryptWithPSK, encryptWithPSK, generateKey } from "./../functions/crypto";

    const mainStore = useMainStore();

    const generatorText = ref("");
    const outputText = ref("");

    console.log("asdasd");
    async function test() {
        const testkey = await generateKey();
        console.log(testkey);
        const clear = "testmessageäßeà";
        const encrypted = await encryptWithPSK(clear, testkey);
        console.log(encrypted);
        const decrypted = await decryptWithPSK(encrypted, testkey);
        console.log(decrypted);

        const testKeyOther = "tSttKbYmHa7EnXqwgWT2AmdCFNk9q5rTq2I12YVHegk=";
        const testEncryptedOther = "qqunM5E4pYoquSLPkWkUDZ3q4TrDPWQT:yM8/8grUR6HEyOPz/szQPgh834WcmkQWiXXzsvP5S23dYQ==";
        console.log("From external", await decryptWithPSK(testEncryptedOther, testKeyOther));
    }
    test();
</script>

<style scoped></style>
