use actix_web::{http::StatusCode, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use log::{error, info};

mod model;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if let Err(_) = std::env::var("RUST_LOG") {
        std::env::set_var("RUST_LOG", "info"); // Set default log level to info
    }
    pretty_env_logger::init();

    let conf = match envy::from_env::<model::Config>() {
        Ok(conf) => conf,
        Err(e) => {
            error!("Fatal: {}", e);
            std::process::exit(1);
        }
    };

    info!(
        "Starting!\n\tPort: {}\n\tStatus: {}\n\tLog requests: {}\n\tType: {:?}\n\tData: {:?}",
        conf.port, conf.status_code, conf.log, conf.action_type, conf.action_data
    );

    let cloned_conf = conf.clone();
    HttpServer::new(move || {
        App::new()
            .data(cloned_conf.clone())
            .default_service(web::route().to(catch_all))
    })
    .bind(format!("0.0.0.0:{}", &conf.port))?
    .run()
    .await
}

fn replace_data(req: HttpRequest, data: String) -> String {
    data.replace("$PATH", req.path()).replace(
        "$HOST",
        req.headers().get("host").unwrap().to_str().unwrap(),
    )
}

async fn catch_all(req: HttpRequest, config: web::Data<model::Config>) -> impl Responder {
    if config.log {
        info!(
            "{} TO {} FROM {}",
            req.method(),
            req.path(),
            req.connection_info().realip_remote_addr().unwrap()
        );
    }

    if config.action_type == model::ActionType::Redirect {
        HttpResponse::build(StatusCode::PERMANENT_REDIRECT)
            .header("LOCATION", config.action_data.to_string())
            .body("")
    } else {
        HttpResponse::build(
            StatusCode::from_u16(config.status_code).unwrap_or(StatusCode::NOT_FOUND),
        )
        .content_type(config.action_type.content_type())
        .body(replace_data(
            req,
            config
                .action_type
                .build_data(config.action_data.to_string()),
        ))
    }
}
