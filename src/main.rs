use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde_json::json;

// Handler for the root path
// This is an attribute macro that defines a GET route for "/"
#[get("/")]
// Attribute macro must attach to the next item, which is the function below
async fn root() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "message": "Silence is golden!"
    }))
}

#[actix_web::main] // Attribute macro to set up the Actix web server
// The main function is the entry point of the application
// Returns a Result indicating success or failure of the server startup
async fn main() -> std::io::Result<()> {
    // Set up the HTTP server with the root route
    HttpServer::new(|| App::new().service(root))
    // Bind the server to localhost on port 8080
        .bind(("127.0.0.1", 8080))? 
        .run() // Start the server
        .await // Await the server's completion
}

