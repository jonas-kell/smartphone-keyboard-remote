use base64::{
    alphabet,
    engine::{self, general_purpose, DecodePaddingMode},
    Engine as _,
};
use sodiumoxide;
use sodiumoxide::crypto::secretbox;

const BASE_64_ENGINE: engine::GeneralPurpose = engine::GeneralPurpose::new(
    &alphabet::STANDARD,
    general_purpose::PAD
        .with_encode_padding(true)
        .with_decode_padding_mode(DecodePaddingMode::Indifferent),
);

pub fn base_64_encode_bytes_to_string(input: &Vec<u8>) -> String {
    BASE_64_ENGINE.encode(input)
}

pub fn base_64_decode_string_to_bytes(input: &str) -> Vec<u8> {
    BASE_64_ENGINE.decode(input).unwrap()
}

pub fn generate_key() -> String {
    sodiumoxide::init().unwrap();

    base_64_encode_bytes_to_string(&secretbox::gen_key().as_ref().to_vec())
}

pub fn encrypt_with_psk(plaintext: &str, psk: &str) -> String {
    sodiumoxide::init().unwrap();

    // Generate a nonce (random number) for encryption
    let nonce = secretbox::gen_nonce();

    // Encrypt the plaintext with a pre-shared key (psk) using secretbox (Sodium's authenticated encryption)
    let key = secretbox::Key::from_slice(&*base_64_decode_string_to_bytes(psk))
        .ok_or("Invalid PSK")
        .unwrap();
    let message_bytes_utf8 = plaintext.as_bytes().to_vec();
    let ciphertext = secretbox::seal(&message_bytes_utf8, &nonce, &key);

    // Return the nonce and ciphertext as a base64 encoded string
    format!(
        "{}:{}",
        base_64_encode_bytes_to_string(&nonce.as_ref().to_vec()),
        base_64_encode_bytes_to_string(&ciphertext)
    )
}

pub fn decrypt_with_psk(encrypted_data: &str, psk: &str) -> String {
    // Decode the base64-encoded nonce and ciphertext
    let parts: Vec<&str> = encrypted_data.split(':').collect();
    let nonce = base_64_decode_string_to_bytes(&parts[0]);
    let ciphertext = base_64_decode_string_to_bytes(&parts[1]);

    sodiumoxide::init().unwrap();

    // Decrypt the data using the pre-shared key (psk) and nonce
    let key = secretbox::Key::from_slice(&*base_64_decode_string_to_bytes(psk))
        .ok_or("Invalid PSK")
        .unwrap();
    let decrypted = secretbox::open(
        &ciphertext,
        &secretbox::Nonce::from_slice(&nonce).unwrap(),
        &key,
    )
    .unwrap();

    String::from(String::from_utf8_lossy(&decrypted))
}
