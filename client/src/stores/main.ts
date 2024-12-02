import { defineStore } from "pinia";
import { computed, ref, watch } from "vue";
import { decryptWithPSK, encryptWithPSK } from "../functions/crypto";

const storageKeyIP = "LOCALSTORAGEKEYIP";
const storageKeyPSK = "LOCALSTORAGEKEYPSK";

const port = "7865";

async function communicateWithBackendEncryptedInternal(
    ip: string,
    psk: string,
    method: string,
    payload: string
): Promise<[string, string]> {
    try {
        const response = await fetch(`http://${ip}:${port}/external/command`, {
            method: "PUT", // HTTP method
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                method: await encryptWithPSK(method, psk),
                payload: await encryptWithPSK(payload, psk),
            }),
        });
        if (response.ok) {
            // we can assume that the communication is setup correctly, otherwise no ok
            const res = await response.json();
            const methodResponse = await decryptWithPSK(res["method"], psk);
            const payloadResponse = await decryptWithPSK(res["payload"], psk);
            return [methodResponse, payloadResponse];
        } else {
            return ["response_not_ok", String(response.status)];
        }
    } catch (error) {
        console.log(error);
        return ["error", String(error)];
    }
}

async function communicateWithBackendUnencryptedLocalhost(method: string, payload: string): Promise<[string, string]> {
    try {
        const response = await fetch(`/internal/command`, {
            method: "PUT", // HTTP method
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                method: method,
                payload: payload,
            }),
        });
        if (response.ok) {
            // we can assume that the communication is unencrypted, as localhost is enforced
            const res = await response.json();
            const methodResponse = res["method"];
            const payloadResponse = res["payload"];
            return [methodResponse, payloadResponse];
        } else {
            return ["response_not_ok", String(response.status)];
        }
    } catch (error) {
        console.log(error);
        return ["error", String(error)];
    }
}

export default defineStore("main", () => {
    const typePending = ref(true);
    const localhostAccess = ref(false);

    const ip = ref(localStorage.getItem(storageKeyIP));
    watch(ip, () => {
        if (ip.value == null) {
            localStorage.removeItem(storageKeyIP);
        } else {
            localStorage.setItem(storageKeyIP, ip.value);
        }
    });
    const psk = ref(localStorage.getItem(storageKeyPSK));
    watch(psk, () => {
        if (psk.value == null) {
            localStorage.removeItem(storageKeyPSK);
        } else {
            localStorage.setItem(storageKeyPSK, psk.value);
        }
    });

    async function initLocalhostType() {
        let [responseMethod, responsePayload] = await communicateWithBackendUnencryptedLocalhost("check_local", "");

        if (responseMethod == "ack_local") {
            localhostAccess.value = true;
        } else {
            console.error(responseMethod, responsePayload);

            // on init if you are external device, check if the full connection is already set up
            tryValidateConnection();
        }
        typePending.value = false; // Always set pending to false after the request
    }
    initLocalhostType();

    const serverConnectionEstablished = ref(false);
    async function tryValidateConnection(pskTest: string | null = null) {
        serverConnectionEstablished.value = false;
        let usePSK = psk.value;
        if (pskTest != null && pskTest) {
            usePSK = pskTest;
        }
        if (usePSK != null && usePSK && ip.value != null && ip.value) {
            let [responseMethod, responsePayload] = await communicateWithBackendEncryptedInternal(
                ip.value,
                usePSK,
                "check_psk",
                ""
            );

            if (responseMethod == "ack_psk") {
                serverConnectionEstablished.value = true;
                if (usePSK != psk.value) {
                    psk.value = usePSK;
                }
            } else {
                console.error(responseMethod, responsePayload);
            }
        }
    }
    async function checkedEncryptedBackendCommunication(method: string, payload: string, checkResponseMethod: string) {
        if (serverConnectionEstablished.value) {
            let usePSK = psk.value as string; // if connection established, this cannot be null
            let useIP = ip.value as string; // if connection established, this cannot be null

            let [responseMethod, responsePayload] = await communicateWithBackendEncryptedInternal(useIP, usePSK, method, payload);

            if (responseMethod == checkResponseMethod) {
                console.log("Successful execution of: ", method);
            } else {
                console.error(responseMethod, responsePayload);
            }
        } else {
            console.error("Can not send commands to server, while connection not established");
        }
    }

    const isLocalhost = computed(() => {
        return !typePending.value && localhostAccess.value;
    });
    const isExternal = computed(() => {
        return !typePending.value && !localhostAccess.value;
    });
    const isReady = computed(() => {
        return !typePending.value;
    });

    return {
        isLocalhost,
        isExternal,
        isReady,
        psk,
        ip,
        serverConnectionEstablished,
        tryValidateConnection,
        communicateWithBackendUnencryptedLocalhost,
        checkedEncryptedBackendCommunication,
    };
});
