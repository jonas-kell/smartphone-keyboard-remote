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

    const mainStore = useMainStore();

    const generatorText = ref("");
    const outputText = ref("");
</script>

<style scoped></style>
