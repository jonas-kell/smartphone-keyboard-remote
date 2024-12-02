<template>
    <h1>Smartphone Keyboard Remote</h1>

    <template v-if="mainStore.isReady">
        <p v-if="mainStore.isLocalhost">You are localhost, you can configure the server here</p>
        <p v-if="mainStore.isExternal">
            You are on an external device: <router-link to="/configure">CONFIGURE your device</router-link>
        </p>
        <template v-if="mainStore.isExternal">
            <h2>
                <span v-if="mainStore.serverConnectionEstablished" style="color: green">Connection Established</span>
                <span v-else style="color: red">Connection Not Established</span>
            </h2>
        </template>
        <template v-if="mainStore.isLocalhost"> <ConfigureLocalhost></ConfigureLocalhost> </template>
    </template>
    <span v-else>Page Type status pending</span>
</template>

<script setup lang="ts">
    import useMainStore from "./../stores/main";
    import ConfigureLocalhost from "./ConfigureLocalhost.vue";

    const mainStore = useMainStore();
</script>

<style scoped></style>
