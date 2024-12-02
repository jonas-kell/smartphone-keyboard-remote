import { defineStore } from "pinia";
import { computed, ref } from "vue";

export default defineStore("main", () => {
    const typePending = ref(true);
    const localhostAccess = ref(false);

    async function initLocalhostType() {
        try {
            const response = await fetch("/internal/check");
            if (response.ok) {
                localhostAccess.value = true;
            }
        } catch (error) {
            console.log(error);
        } finally {
            typePending.value = false; // Always set pending to false after the request
        }
    }
    initLocalhostType();

    const isLocalhost = computed(() => {
        return !typePending.value && localhostAccess.value;
    });
    const isExternal = computed(() => {
        return !typePending.value && !localhostAccess.value;
    });

    return {
        isLocalhost,
        isExternal,
    };
});
