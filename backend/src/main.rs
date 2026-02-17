use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CreateProduct {
    name: String,
    quantity: i32,
}

#[derive(Serialize)]
struct ApiResponse {
    message: String,
}

#[post("/api/products")]
async fn create_product(
    product: web::Json<CreateProduct>
) -> impl Responder {

    // Simulating database insert
    println!(
        "Product received: {} | Quantity: {}",
        product.name,
        product.quantity
    );

    HttpResponse::Ok().json(ApiResponse {
        message: format!("Product {} created successfully", product.name),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(create_product)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
