<template>
    <p>Type Here:</p>
    <input :type="passwordField ? 'password' : 'text'" v-model="textToType" @blur="typeText" />
    <button @click="enter">Enter</button>
    <br />
    <button @click="backspace">Delete</button>
    <input type="checkbox" id="password" v-model="passwordField" />
    <label for="password">Password</label>
    <br />
    <br />
    <br />
    <br />
    Focus this to send keystrokes live
    <br />
    <input
        type="text"
        :style="{
            'background-color': liveTypeFocus ? 'green' : 'red',
        }"
        @focusin="liveTypeFocus = true"
        @focusout="liveTypeFocus = false"
    />
</template>

<script setup lang="ts">
    import useMainStore from "../stores/main";
    import { ref, watch } from "vue";

    const mainStore = useMainStore();

    const textToType = ref("");
    const passwordField = ref(false);

    const liveTypeFocus = ref(false);

    function preventAndSend(event: Event) {
        let eventTyped = event as KeyboardEvent;
        eventTyped.preventDefault();

        // console.log(`${eventTyped.type}: key="${eventTyped.key}", code="${eventTyped.code}", meta=${eventTyped.metaKey}`);
        const method = `key_various_${eventTyped.type == "keyup" ? "up" : eventTyped.type == "keydown" ? "down" : "unknown"}`;

        let codeToSend = eventTyped.code;
        if (codeToSend == "NumpadEnter") {
            codeToSend = "Super"; // Numpad Enter sends super, as we can not reliably detect that otherwise
        }
        mainStore.checkedEncryptedBackendCommunication(method, codeToSend, "ack_key");
    }
    watch(liveTypeFocus, () => {
        if (liveTypeFocus.value) {
            ["keydown", "keypress", "keyup"].forEach((type) => {
                document.addEventListener(type, preventAndSend);
            });
        } else {
            ["keydown", "keypress", "keyup"].forEach((type) => {
                document.removeEventListener(type, preventAndSend);
            });
        }
    });

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
