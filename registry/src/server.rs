use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use clerk_rs::{validators::actix::ClerkMiddleware, ClerkConfiguration};
use trunk_registry::routes::token::new_token;
use trunk_registry::{config, connect, routes};

pub fn routes_config(configuration: &mut web::ServiceConfig) {
    let cfg = config::Config::default();
    let clerk_cfg = ClerkConfiguration::new(None, None, Some(cfg.clerk_secret_key), None);
    configuration
        .service(routes::root::ok)
        .service(routes::extensions::get_all_extensions)
        .service(routes::extensions::publish)
        .service(routes::download::download)
        .service(
            web::scope("/token")
                .wrap(ClerkMiddleware::new(clerk_cfg))
                .service(new_token),
        );
}

pub async fn server() -> std::io::Result<()> {
    env_logger::init();
    // load configurations from environment
    let cfg = config::Config::default();
    let aws_config = aws_config::load_from_env().await;

    let conn = connect(&cfg.database_url)
        .await
        .expect("error connecting to database");

    // run database migrations
    sqlx::migrate!()
        .run(&conn)
        .await
        .expect("error running migrations");

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(conn.clone()))
            .app_data(web::Data::new(cfg.clone()))
            .app_data(web::Data::new(aws_config.clone()))
            .configure(routes_config)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
