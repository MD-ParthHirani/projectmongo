use mongodb::bson::{self, oid::ObjectId};
use serde:: {Serialize};
use warp::{reply::json, Filter, Rejection, Reply};

type WebResult<T> = std::result::Result<T, Rejection>;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

pub async fn health_checker_handler() -> WebResult<impl Reply> {
    const MESSAGE: &str = "Build CRUD API with Rust and MongoDB";

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    Ok(json(response_json))
}

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "api=info");
    }
    pretty_env_logger::init();

    let health_checker = warp::path!("api" / "healthchecker")
        .and(warp::get())
        .and_then(health_checker_handler);

    

    let routes = health_checker.with(warp::log("api"));

    println!(" Server started successfully");
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

/*use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json("Hello from rust and mongoDB")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("localhost", 8080))?
        .run()
        .await
}*/
