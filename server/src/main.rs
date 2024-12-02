use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::{from_fn, Next};
use actix_web::{web, App, Error, HttpResponse, HttpServer, Responder};
use enigo::*;
use error::CustomError;
use include_dir::{include_dir, Dir};
use local_ip_address::local_ip;
use mime_guess::from_path;
use serde::Serialize;
use std::collections::HashMap;
use std::net::IpAddr;
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

const STATIC_FILES: Dir = include_dir!("$CARGO_MANIFEST_DIR/../client/dist");

fn cache_static_files() -> HashMap<String, &'static [u8]> {
    let mut file_map = HashMap::new();

    fn add_files_recursively(dir: &'static Dir, file_map: &mut HashMap<String, &'static [u8]>) {
        for file in dir.files() {
            if let Some(path) = file.path().to_str() {
                file_map.insert(String::from(path), file.contents());
            }
        }

        for subdir in dir.dirs() {
            add_files_recursively(subdir, file_map);
        }
    }

    // Start scanning from the root directory with an empty base path.
    add_files_recursively(&STATIC_FILES, &mut file_map);

    file_map
}

async fn static_handler(
    file_map: web::Data<HashMap<String, &'static [u8]>>,
    path: web::Path<String>,
) -> impl Responder {
    let path = path.into_inner();
    let file_map = file_map.get_ref();
    println!("Serving statically {}", path);

    // Try to resolve the requested path or default to `index.html`.
    let file_path = if path.is_empty() || path == "/" {
        "index.html"
    } else {
        path.as_str()
    };

    if let Some(content) = file_map.get(file_path) {
        let mime_type = from_path(file_path).first_or_octet_stream();
        HttpResponse::Ok()
            .content_type(mime_type.as_ref())
            .body(*content)
    } else {
        HttpResponse::NotFound().body("404 Not Found")
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
        let file_map = web::Data::new(cache_static_files());

        App::new()
            .app_data(file_map)
            .service(
                web::scope(format!("/{}", path_segment).as_str())
                    .route("/{path:.*}", web::get().to(static_handler)),
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
