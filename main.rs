use actix_web::{middleware::Logger, web, App, HttpServer};
use std::fs::File;
use std::io::BufReader;

use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi, Url};

mod api {
    use actix_web::get;

    #[utoipa::path(
        context_path = "/api",
        responses(
            (status = 200, description = "Hello from api", body = String)
        )
    )]
    #[get("/hello")]
    pub(super) async fn hello() -> String {
        "hello from api".to_string()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .unwrap();

    let mut certs_file = BufReader::new(File::open("cert.pem").unwrap());
    let mut key_file = BufReader::new(File::open("key.pem").unwrap());

    // load TLS certs and key
    // to create a self-signed temporary cert for testing:
    // `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
    let tls_certs = rustls_pemfile::certs(&mut certs_file)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let tls_key = rustls_pemfile::pkcs8_private_keys(&mut key_file)
        .next()
        .unwrap()
        .unwrap();

    // set up TLS config options
    let tls_config = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(tls_certs, rustls::pki_types::PrivateKeyDer::Pkcs8(tls_key))
        .unwrap();

    #[derive(OpenApi)]
    #[openapi(paths(api::hello))]
    struct ApiDoc;
    
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(web::scope("/api").service(api::hello))
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").urls(vec![(
                Url::with_primary("api", "/api-docs/openapi.json", true),
                ApiDoc::openapi(),
            )]))
    })
    .bind_rustls_0_23(("127.0.0.1", 8080), tls_config)?
    .run()
    .await
}
