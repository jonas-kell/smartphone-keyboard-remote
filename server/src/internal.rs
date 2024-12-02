use crate::crypto::{decrypt_with_psk, encrypt_with_psk, generate_key};
use crate::env_storage::{read_from_env, update_env_file};
use crate::error::CustomError;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::Next;
use actix_web::{web, Error, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Serialize, Deserialize)]
pub struct CommunicationStruct {
    method: String,
    payload: String,
}

pub async fn internal_route(body: web::Json<CommunicationStruct>) -> impl Responder {
    match body.method.as_str() {
        "check_local" => HttpResponse::Ok().json(CommunicationStruct {
            method: String::from("ack_local"),
            payload: String::from(""),
        }),
        "get_psk" => {
            let stored_psk = match read_from_env("mainpsk") {
                Some(psk) => psk,
                None => {
                    let new_psk = generate_key();
                    update_env_file("mainpsk", &new_psk).unwrap();
                    new_psk
                }
            };

            HttpResponse::Ok().json(CommunicationStruct {
                method: String::from("ret_psk"),
                payload: stored_psk,
            })
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
    match body.method.as_str() {
        _ => HttpResponse::Ok().json(CommunicationStruct {
            method: String::from("unknown_method"),
            payload: String::from(body.method.as_str()),
        }),
    }
}

pub async fn localhost_ip_filter(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let client_ip = req
        .peer_addr()
        .map(|addr| addr.ip())
        .unwrap_or(IpAddr::V4([222, 222, 222, 222].into()));
    println!("Request from IP: {}", client_ip);

    if match client_ip {
        IpAddr::V4(ipv4) => ipv4.is_loopback(),
        _ => false,
    } {
        let res = next.call(req).await?;
        Ok(res)
    } else {
        Err(Error::from(CustomError::new(
            403,
            String::from("Only accessible from localhost"),
        )))
    }
}
