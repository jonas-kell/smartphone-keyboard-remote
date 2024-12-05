use crate::error::CustomError;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::Next;
use actix_web::Error;
use local_ip_address::local_ip;
use std::net::IpAddr;

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

pub fn get_local_ip() -> String {
    match local_ip() {
        Err(err) => String::from(format!("{}", err)),
        Ok(add) => add.to_string(),
    }
}
