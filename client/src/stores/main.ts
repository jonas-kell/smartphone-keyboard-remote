import { defineStore } from "pinia";
import { ref } from "vue";

export default defineStore("keys", () => {
    const stuff = ref("");

    return {
        stuff,
    };
});
