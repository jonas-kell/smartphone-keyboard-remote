<template>
    <p>Type Here:</p>
    <input type="text" v-model="textToType" @blur="typeText" />
    <button @click="backspace">Delete</button>
</template>

<script setup lang="ts">
    import useMainStore from "../stores/main";
    import { ref } from "vue";

    const mainStore = useMainStore();

    const textToType = ref("");

    async function backspace() {
        mainStore.checkedEncryptedBackendCommunication("key_backspace", "", "ack_key");
    }
    async function typeText() {
        await mainStore.checkedEncryptedBackendCommunication("single_text", textToType.value, "ack_text");
        textToType.value = "";
    }
</script>

<style scoped></style>
