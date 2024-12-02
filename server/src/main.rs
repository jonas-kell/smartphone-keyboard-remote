use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::{from_fn, Next};
use actix_web::{web, App, Error, HttpResponse, HttpServer, Responder};
use enigo::*;
use error::CustomError;
use local_ip_address::local_ip;
use serde::Serialize;
use std::net::IpAddr;
use std::thread;
use std::time::Duration;

mod error;
mod static_files;

#[derive(Serialize)]
struct MyResponse {
    message: String,
}
async fn dynamic_route() -> impl Responder {
    paste_test();

    HttpResponse::Ok().json(MyResponse {
        message: String::from("This is a dynamic JSON response!"),
    })
}

fn paste_test() {
    let mut enigo = Enigo::new();

    println!("pasting");
    thread::sleep(Duration::from_secs(1));
    // Simulate pressing a combination: Ctrl+V
    enigo.key_down(Key::Control);
    enigo.key_click(Key::Layout('v'));
    enigo.key_up(Key::Control);
    println!("end");
}

async fn localhost_ip_filter(
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server_host = "0.0.0.0";
    let server_port = "7865";
    let path_segment = "smartphone-keyboard-remote";
    println!("Server started on {}:{}", server_host, server_port);
    println!(
        "Localhost access from http://{}:{}/{}/",
        "127.0.0.1", server_port, path_segment
    );
    match local_ip() {
        Err(err) => println!("{}", err),
        Ok(add) => {
            println!(
                "LAN access from http://{}:{}/{}/",
                add, server_port, path_segment
            )
        }
    }

    HttpServer::new(move || {
        let file_map = web::Data::new(static_files::cache_static_files());

        App::new()
            .app_data(file_map)
            .service(
                web::scope(format!("/{}", path_segment).as_str())
                    .route("/{path:.*}", web::get().to(static_files::static_handler)),
            )
            .service(
                web::scope("/internal")
                    .route("/json", web::get().to(dynamic_route))
                    .wrap(from_fn(localhost_ip_filter)),
            )
            .service(web::redirect("/", format!("/{}/", path_segment)))
    })
    .bind(format!("{}:{}", server_host, server_port))?
    .run()
    .await
}
