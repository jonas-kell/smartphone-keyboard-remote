use actix_web::middleware::from_fn;
use actix_web::{web, App, HttpServer};
use crypto::{decrypt_with_psk, encrypt_with_psk, generate_key};
use local_ip_address::local_ip;

mod crypto;
mod env_storage;
mod error;
mod execution;
mod internal;
mod static_files;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let key = generate_key();
    let message = "testmessageäßeà";
    println!("Key: {}", key);
    let encrypted = encrypt_with_psk(message, &key);
    println!("Encrypted: {}", encrypted);
    let decrypted = decrypt_with_psk(&encrypted, &key);
    println!("Decrypted: {}", decrypted);

    let test_key_str = "VTkVklZkI9AWVDsLKyiQ1x7pLiubabKxkqIXd3CokRc=";
    let test_encrypted_str =
        "NyJx/bto4F8wM3wD+hhm0b6h1pfm8NJz:pudkeMtQnd3p7FPjwiGuBZulrR5c7BhGGAjeunl2aizaGw==";
    let test_decrypt = decrypt_with_psk(test_encrypted_str, test_key_str);
    println!("Decrypted from other encryption: {}", test_decrypt);

    println!(
        "got: {}",
        env_storage::read_from_env("test").unwrap_or_default()
    );

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
                    .route("/check", web::get().to(internal::check_local_route))
                    .wrap(from_fn(internal::localhost_ip_filter)),
            )
            .service(web::redirect("/", format!("/{}/", path_segment)))
    })
    .bind(format!("{}:{}", server_host, server_port))?
    .run()
    .await
}
