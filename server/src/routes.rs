use crate::crypto::{decrypt_with_psk, encrypt_with_psk, generate_key};
use crate::env_storage::{read_from_env, update_env_file};
use crate::execution::{keyboard_basic_text, keyboard_delete, keyboard_enter, keyboard_various};
use crate::ip;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CommunicationStruct {
    method: String,
    payload: String,
}

fn get_or_regen_psk() -> String {
    match read_from_env("mainpsk") {
        Some(psk) => psk,
        None => {
            let new_psk = generate_key();
            update_env_file("mainpsk", &new_psk).unwrap();
            new_psk
        }
    }
}

pub async fn internal_route(body: web::Json<CommunicationStruct>) -> impl Responder {
    match body.method.as_str() {
        "check_local" => HttpResponse::Ok().json(CommunicationStruct {
            method: String::from("ack_local"),
            payload: String::from(""),
        }),
        "get_psk" => {
            let stored_psk = get_or_regen_psk();

            HttpResponse::Ok().json(CommunicationStruct {
                method: String::from("ret_psk"),
                payload: stored_psk,
            })
        }
        "get_ip" => HttpResponse::Ok().json(CommunicationStruct {
            method: String::from("ret_ip"),
            payload: ip::get_local_ip(),
        }),
        "shutdown_server" => {
            // no bother implementing graceful shutdown...
            println!("Requested server shutdown");
            std::process::exit(0);
        }
        "regenerate_psk" => {
            let new_psk = generate_key();
            update_env_file("mainpsk", &new_psk).unwrap();

            HttpResponse::Ok().json(CommunicationStruct {
                method: String::from("ret_psk"),
                payload: new_psk,
            })
        }
        _ => HttpResponse::Ok().json(CommunicationStruct {
            method: String::from("unknown_method"),
            payload: String::from(body.method.as_str()),
        }),
    }
}

pub async fn external_route(body: web::Json<CommunicationStruct>) -> impl Responder {
    let psk = get_or_regen_psk();
    let decrypted_method = decrypt_with_psk(body.method.as_str(), &psk);
    let decrypted_payload = decrypt_with_psk(body.payload.as_str(), &psk);

    match decrypted_method.as_str() {
        "check_psk" => HttpResponse::Ok().json(CommunicationStruct {
            method: encrypt_with_psk("ack_psk", &psk),
            payload: encrypt_with_psk("", &psk),
        }),
        "single_text" => {
            let text = decrypted_payload;

            keyboard_basic_text(&text);

            HttpResponse::Ok().json(CommunicationStruct {
                method: encrypt_with_psk("ack_text", &psk),
                payload: encrypt_with_psk("", &psk),
            })
        }
        "text_enter" => {
            let text = decrypted_payload;

            keyboard_basic_text(&text);
            keyboard_enter();

            HttpResponse::Ok().json(CommunicationStruct {
                method: encrypt_with_psk("ack_text", &psk),
                payload: encrypt_with_psk("", &psk),
            })
        }
        "key_backspace" => {
            keyboard_delete();

            HttpResponse::Ok().json(CommunicationStruct {
                method: encrypt_with_psk("ack_key", &psk),
                payload: encrypt_with_psk("", &psk),
            })
        }
        "key_enter" => {
            keyboard_enter();

            HttpResponse::Ok().json(CommunicationStruct {
                method: encrypt_with_psk("ack_key", &psk),
                payload: encrypt_with_psk("", &psk),
            })
        }
        "key_various_down" => {
            let key_code = decrypted_payload;
            keyboard_various(&key_code, true);

            HttpResponse::Ok().json(CommunicationStruct {
                method: encrypt_with_psk("ack_key", &psk),
                payload: encrypt_with_psk("", &psk),
            })
        }
        "key_various_up" => {
            let key_code = decrypted_payload;
            keyboard_various(&key_code, false);

            HttpResponse::Ok().json(CommunicationStruct {
                method: encrypt_with_psk("ack_key", &psk),
                payload: encrypt_with_psk("", &psk),
            })
        }
        "key_super_down" => {
            keyboard_various("super", true);

            HttpResponse::Ok().json(CommunicationStruct {
                method: encrypt_with_psk("ack_key", &psk),
                payload: encrypt_with_psk("", &psk),
            })
        }
        "key_super_up" => {
            keyboard_various("super", false);

            HttpResponse::Ok().json(CommunicationStruct {
                method: encrypt_with_psk("ack_key", &psk),
                payload: encrypt_with_psk("", &psk),
            })
        }
        _ => HttpResponse::Ok().json(CommunicationStruct {
            method: String::from("unknown_method"),
            payload: decrypted_method,
        }),
    }
}
