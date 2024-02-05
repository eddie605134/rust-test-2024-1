// use actix_web::{web, App, HttpResponse, HttpServer, Responder};

// async fn greet() -> impl Responder {
//     let mut client = "Hello world!";
//     HttpResponse::Ok().body(client)
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .route("/", web::get().to(greet))
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }

fn day_1() {
    println!("Hello rust!");
}
