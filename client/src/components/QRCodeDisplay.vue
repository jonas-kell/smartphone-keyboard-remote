<template>
    <div :style="{ visibility: props.text == '' || props.text == null ? 'hidden' : 'unset' }">
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

    watch(props, () => {
        generateQRCode(props.text);
    });

    const generateQRCode = async (text: string) => {
        if (text != null && text != "" && text) {
            try {
                if (qrcodeCanvas.value) {
                    await QRCode.toCanvas(qrcodeCanvas.value, text);
                } else {
                    console.error("Canvas not found to write to");
                }
            } catch (error) {
                console.error("Error generating QR Code:", error);
            }
        }
    };
</script>
