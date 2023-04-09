use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Task {
    id: usize,
    title: String,
    completed: bool,
}

async fn index() -> impl Responder {
    let tasks = vec![
        Task {
            id: 1,
            title: "Create a Rust web server".to_string(),
            completed: false,
        },
        Task {
            id: 2,
            title: "Deploy the Rust web server".to_string(),
            completed: false,
        },
    ];

    HttpResponse::Ok().json(tasks)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
