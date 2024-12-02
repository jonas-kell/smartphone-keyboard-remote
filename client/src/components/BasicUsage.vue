<template>
    <p>Type Here:</p>
    <input type="text" v-model="textToType" @blur="typeText" />
    <button @click="enter">Enter</button>
    <br />
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
    async function enter() {
        if (textToType.value) {
            const copy = textToType.value;
            textToType.value = "";
            await mainStore.checkedEncryptedBackendCommunication("text_enter", copy, "ack_text");
        } else {
            mainStore.checkedEncryptedBackendCommunication("key_enter", "", "ack_key");
        }
    }
    function typeText() {
        setTimeout(async () => {
            if (textToType.value) {
                await mainStore.checkedEncryptedBackendCommunication("single_text", textToType.value, "ack_text");
                textToType.value = "";
            }
        }, 400); // so that enter has precedence TODO not so nice
    }
</script>

<style scoped></style>
