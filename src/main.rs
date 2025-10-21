use actix_web::{get, App, HttpResponse, HttpServer, Responder};

// Handler for the root path
// This is an attribute macro that defines a GET route for "/"
#[get("/")]
// Attribute macro must attach to the next item, which is the function below
async fn root() -> impl Responder{
    HttpResponse::Ok().body("Silence is golden.")
}


#[actix_web::main] // Attribute macro to set up the Actix web server
async fn main() {
    // Start the HTTP server
    HttpServer::new(|| { 
        App::new() // Create a new Actix web application
            .service(root) // Register the root handler as a service
    })
    .bind("127.0.0.1:8080") // Bind the server to localhost on port 8080
    .unwrap() // Unwrap the Result to handle any binding errors
    .run() // Start the HTTP server
    .await // Await the server to run 
    .unwrap(); // Unwrap the Result to handle any runtime errors
}

