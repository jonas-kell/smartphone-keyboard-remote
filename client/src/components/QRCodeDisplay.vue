<template>
    <div v-if="props.text != '' && props.text != null">
        <canvas ref="qrcodeCanvas"></canvas>
    </div>
</template>

<script setup lang="ts">
    import { ref, watch } from "vue";
    import QRCode from "qrcode";

    const props = defineProps<{
        text: string;
    }>();

    const qrcodeCanvas = ref(null);

    watch([props], () => {
        console.log("asd");
        generateQRCode(props.text);
    });

    const generateQRCode = async (text: string) => {
        try {
            if (qrcodeCanvas.value) {
                await QRCode.toCanvas(qrcodeCanvas.value, text);
            }
        } catch (error) {
            console.error("Error generating QR Code:", error);
        }
    };
</script>
