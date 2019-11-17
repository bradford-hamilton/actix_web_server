use actix_web::{web, App, HttpResponse, HttpServer, Responder, get};

// One way to implement a responder
fn index() -> impl Responder {
    HttpResponse::Ok().body("Hey there")
}

// Another way is with macros
#[get("/hello")]
fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello yourself")
}

fn main() {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index)) // Register the index responder function
            .service(hello) // Register the hello using attribute macro
    })
    .bind("127.0.0.1:3000")
    .unwrap()
    .run()
    .unwrap();
}
