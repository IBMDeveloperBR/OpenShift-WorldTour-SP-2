use actix_web::{get, middleware, web, App, HttpRequest, HttpResponse, HttpServer};

async fn index_async(req: HttpRequest) -> &'static str {
    println!("REQ: {:?}", req);
    "Hello world!\r\n"
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.2"))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/")
                    .wrap(
                        middleware::DefaultHeaders::new().header("X-Version-R2", "0.3"),
                    )
                    .default_service(
                        web::route().to(|| HttpResponse::MethodNotAllowed()),
                    )
                    .route(web::get().to(index_async))
            )
    })
    .bind("0.0.0.0:8080")?
    .workers(1)
    .run()
    .await
}
