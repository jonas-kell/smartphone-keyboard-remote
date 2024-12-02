use actix_files;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::{from_fn, Next};
use actix_web::{web, App, Error, HttpResponse, HttpServer, Responder};
use enigo::*;
use error::CustomError;
use include_dir::{include_dir, Dir};
use local_ip_address::local_ip;
use serde::Serialize;
use std::net::IpAddr;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

mod error;

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

    if is_local_ip(client_ip) {
        let res = next.call(req).await?;
        Ok(res)
    } else {
        Err(Error::from(CustomError::new(
            403,
            String::from("Only accessible from localhost"),
        )))
    }
}

fn is_local_ip(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(ipv4) => ipv4.is_loopback(),
        _ => false,
    }
}

const STATIC_FILES: Dir = include_dir!("$CARGO_MANIFEST_DIR/../client/dist");

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
        App::new()
            .service(
                actix_files::Files::new(
                    format!("/{}", path_segment).as_str(),
                    PathBuf::from(STATIC_FILES.path()),
                )
                .index_file("index.html"),
            )
            .service(
                web::scope("/internal/")
                    .route("/json", web::get().to(dynamic_route))
                    .wrap(from_fn(localhost_ip_filter)),
            )
            .service(web::redirect("/", format!("/{}/", path_segment)))
    })
    .bind(format!("{}:{}", server_host, server_port))?
    .run()
    .await
}
