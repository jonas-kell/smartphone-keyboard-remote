use actix_files as fs;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use enigo::*;
use serde::Serialize;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

// Struct for JSON response
#[derive(Serialize)]
struct MyResponse {
    message: String,
}

// Dynamic route handler
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Path to the folder containing static files
    let static_folder_path = PathBuf::from("./../client/dist");

    let server_host = "127.0.0.1";
    let server_port = "7865";
    let path_segment = "smartphone-keyboard-remote";
    println!(
        "Server started on http://{}:{}/{}/",
        server_host, server_port, path_segment
    );

    HttpServer::new(move || {
        App::new()
            .service(
                fs::Files::new(
                    format!("/{}/", path_segment).as_str(),
                    static_folder_path.clone(),
                )
                .index_file("index.html"),
            )
            .route("/json", web::get().to(dynamic_route))
    })
    .bind(format!("{}:{}", server_host, server_port))?
    .run()
    .await
}
