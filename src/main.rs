use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use std::sync::{Arc, Mutex};

// Shared state for storing screenshot data
type ScreenshotData = Arc<Mutex<Option<Vec<u8>>>>

// Handler for the root endpoint
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Screen share server")
}

// Handler for POST /screenshot
async fn post_screenshot(
    data: web::Bytes, 
    state: web::Data<ScreenshotData>
) -> impl Responder {
    let mut screenshot_data = state.lock().unwrap();
    *screenshot_data = Some(data.to_vec());
    println!("Screenshot data received and stored.");
    HttpResponse::Ok().json(serde_json::json!({ "message": "Screenshot data received and stored successfully" }))
}

// Handler for GET /screenshot
async fn get_screenshot(state: web::Data<ScreenshotData>) -> impl Responder {
    let screenshot_data = state.lock().unwrap();
    if let Some(ref data) = *screenshot_data {
        println!("Sent screenshot data.");
        HttpResponse::Ok()
            .content_type("application/octet-stream")
            .body(data.clone())
    } else {
        HttpResponse::NotFound().json(serde_json::json!({ "error": "No screenshot data available" }))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Shared state for screenshot data
    let screenshot_data = Arc::new(Mutex::new(None::<Vec<u8>>));

    let bind_address = "0.0.0.0:8080"; // Bind to all interfaces on port 8080
    println!("Starting server on {}", bind_address);

    // Start the server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(screenshot_data.clone())) // Pass shared state
            .route("/", web::get().to(root))
            .route("/screenshot", web::post().to(post_screenshot))
            .route("/screenshot", web::get().to(get_screenshot))
    })
    .bind(bind_address)?
    .run()
    .await
}
