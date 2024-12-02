use actix_cors::Cors;
use actix_web::http::header;
use actix_web::middleware::from_fn;
use actix_web::{web, App, HttpServer};
use local_ip_address::local_ip;

mod crypto;
mod env_storage;
mod error;
mod execution;
mod internal;
mod static_files;

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

        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "PUT"])
            .allowed_headers(vec![header::CONTENT_TYPE, header::ACCEPT])
            .max_age(3600);

        App::new()
            .app_data(file_map)
            .wrap(cors)
            .service(
                web::scope(format!("/{}", path_segment).as_str())
                    .route("/{path:.*}", web::get().to(static_files::static_handler)),
            )
            .service(
                web::scope("/internal")
                    .route("/command", web::put().to(internal::internal_route))
                    .wrap(from_fn(internal::localhost_ip_filter)),
            )
            .service(
                web::scope("/external").route("/command", web::put().to(internal::external_route)),
            )
            .service(web::redirect("/", format!("/{}/", path_segment)))
    })
    .bind(format!("{}:{}", server_host, server_port))?
    .run()
    .await
}
