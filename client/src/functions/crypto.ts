import sodium from "libsodium-wrappers";
import { Buffer } from "buffer";

function base64decodeStringToBytes(base64: string) {
    const binString = atob(base64);
    return Uint8Array.from(binString, (m) => m.charCodeAt(0));
}

function base64EncodeBytesToString(bytes: Uint8Array) {
    const binString = Array.from(bytes, (byte) => String.fromCharCode(byte)).join("");
    return btoa(binString);
}

export async function generateKey(): Promise<string> {
    await sodium.ready; // Initialize sodium

    return base64EncodeBytesToString(await sodium.randombytes_buf(sodium.crypto_secretbox_KEYBYTES));
}

export async function encryptWithPSK(plaintext: string, psk: string): Promise<string> {
    const pskbuf = Buffer.from(base64decodeStringToBytes(psk));

    await sodium.ready; // Initialize sodium

    // Generate a nonce (random number)
    const nonce = sodium.randombytes_buf(sodium.crypto_secretbox_NONCEBYTES);

    // Encrypt the plaintext with the pre-shared key (psk)
    const ciphertext = sodium.crypto_secretbox_easy(Buffer.from(plaintext, "utf8"), nonce, pskbuf);

    // Return the nonce and ciphertext as a base64 encoded string
    return `${base64EncodeBytesToString(nonce)}:${base64EncodeBytesToString(ciphertext)}`;
}

export async function decryptWithPSK(encryptedData: string, psk: string): Promise<string> {
    await sodium.ready; // Initialize sodium

    const pskbuf = Buffer.from(base64decodeStringToBytes(psk));

    const parts = encryptedData.split(":");
    if (parts.length !== 2) {
        throw new Error("Invalid encrypted data format.");
    }

    // Decode base64 encoded nonce and ciphertext
    const nonce = Buffer.from(base64decodeStringToBytes(parts[0]));
    const ciphertext = Buffer.from(base64decodeStringToBytes(parts[1]));

    // Decrypt the data using the pre-shared key (psk)
    const decrypted = sodium.crypto_secretbox_open_easy(ciphertext, nonce, pskbuf);

    if (!decrypted) {
        throw new Error("Decryption failed.");
    }

    return Buffer.from(decrypted).toString("utf8");
}
