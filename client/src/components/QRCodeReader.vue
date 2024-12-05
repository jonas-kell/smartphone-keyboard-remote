<template>
    <div>
        <select v-model="selectedConstraints">
            <option v-for="option in constraintOptions" :key="option.label" :value="option.constraints">
                {{ option.label }}
            </option>
        </select>

        <p class="error" v-if="error != '' && error">{{ error }}</p>

        <div
            :style="{
                display: cameraOnceAvailable ? 'unset' : 'none',
            }"
        >
            <qrcode-stream
                :constraints="selectedConstraints"
                :track="paintOutline"
                :formats="barcodeFormats"
                @error="onError"
                @detect="onDetect"
                @camera-on="onCameraReady"
            />
        </div>
    </div>
</template>

<script setup lang="ts">
    // https://gruhn.github.io/vue-qrcode-reader/demos/FullDemo.html

    interface DetectedBarcode {
        boundingBox: DOMRectReadOnly;
        cornerPoints: {
            x: number;
            y: number;
        }[];
        format: BarcodeFormat;
        rawValue: string;
    }

    import { ref } from "vue";
    import { QrcodeStream, BarcodeFormat } from "vue-qrcode-reader";

    const emit = defineEmits<{
        (event: "scanned", value: string): void;
    }>();

    /*** detection handling ***/

    const result = ref("");

    function onDetect(detectedCodes: DetectedBarcode[]) {
        console.log(detectedCodes);
        result.value = JSON.stringify(detectedCodes.map((code) => code.rawValue));

        emit("scanned", detectedCodes[0].rawValue);
    }

    /*** select camera ***/

    const selectedConstraints = ref({ facingMode: "environment" });
    const defaultConstraintOptions: { label: string; constraints: {} }[] = [
        { label: "rear camera", constraints: { facingMode: "environment" } },
        { label: "front camera", constraints: { facingMode: "user" } },
    ];
    const constraintOptions = ref(defaultConstraintOptions);

    const cameraOnceAvailable = ref(false);

    async function onCameraReady() {
        cameraOnceAvailable.value = true;

        // NOTE: on iOS we can't invoke `enumerateDevices` before the user has given
        // camera access permission. `QrcodeStream` internally takes care of
        // requesting the permissions. The `camera-on` event should guarantee that this
        // has happened.
        const devices = await navigator.mediaDevices.enumerateDevices();
        const videoDevices = devices.filter(({ kind }) => kind === "videoinput");

        constraintOptions.value = [
            ...defaultConstraintOptions,
            ...videoDevices.map(({ deviceId, label }) => ({
                label: `${label} (ID: ${deviceId})`,
                constraints: { deviceId },
            })),
        ];

        error.value = "";
    }

    /*** track function ***/

    function paintOutline(detectedCodes: DetectedBarcode[], ctx: CanvasRenderingContext2D) {
        for (const detectedCode of detectedCodes) {
            const [firstPoint, ...otherPoints] = detectedCode.cornerPoints;

            ctx.strokeStyle = "red";

            ctx.beginPath();
            ctx.moveTo(firstPoint.x, firstPoint.y);
            for (const { x, y } of otherPoints) {
                ctx.lineTo(x, y);
            }
            ctx.lineTo(firstPoint.x, firstPoint.y);
            ctx.closePath();
            ctx.stroke();
        }
    }

    /*** barcode formats ***/

    const barcodeFormats: BarcodeFormat[] = ["qr_code"];

    /*** error handling ***/

    const error = ref("");

    function onError(err: { value: string; name: string; message: string }) {
        error.value = `[${err.name}]: `;

        if (err.name === "NotAllowedError") {
            error.value += "you need to grant camera access permission";
        } else if (err.name === "NotFoundError") {
            error.value += "no camera on this device";
        } else if (err.name === "NotSupportedError") {
            error.value += "secure context required (HTTPS, localhost)";
        } else if (err.name === "NotReadableError") {
            error.value += "is the camera already in use?";
        } else if (err.name === "OverconstrainedError") {
            error.value += "installed cameras are not suitable";
        } else if (err.name === "StreamApiNotSupportedError") {
            error.value += "Stream API is not supported in this browser";
        } else if (err.name === "InsecureContextError") {
            error.value += "Camera access is only permitted in secure context. Use HTTPS or localhost rather than HTTP.";
        } else {
            error.value += err.message;
        }
    }
</script>

<style scoped>
    .error {
        font-weight: bold;
        color: red;
    }
    .barcode-format-checkbox {
        margin-right: 10px;
        white-space: nowrap;
        display: inline-block;
    }
</style>
