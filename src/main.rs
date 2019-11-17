use actix_web::{web, App, HttpResponse, HttpServer, Responder, get};
use std::sync::Mutex;

// One way to implement a responder
fn index() -> impl Responder {
    HttpResponse::Ok().body("Hey there")
}

// Another way is with macros
#[get("/hello")]
fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello yourself")
}

struct AppStateWithCounter {
    counter: Mutex<i32>,
}

fn counter_handler(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    
    *counter += 1; // Access counter inside MutexGuard

    format!("Request number: {}", counter)
}

fn main() {
    let counter = web::Data::new(AppStateWithCounter{
        counter: Mutex::new(0),
    });

    HttpServer::new(move || { // Move counter into closure
        App::new()
            .register_data(counter.clone())
            .route("/", web::get().to(index)) // Register the index responder function
            .route("/counter", web::get().to(counter_handler)) // Register route /counter to counter
            .service(hello) // Register the hello using attribute macro
    })
    .bind("127.0.0.1:3000")
    .unwrap()
    .run()
    .unwrap();
}
