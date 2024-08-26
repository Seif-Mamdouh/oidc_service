use actix_web::{web, App, HttpServer, Responder};
use jsonwebtoken::{encode, decode, EncodingKey, DecodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    iat: usize,
}

async fn token_endpoint() -> impl Responder {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as usize;
    
    let claims = Claims {
        sub: "github-action".to_string(),
        exp: now + 3600, // Token expires in 1 hour
        iat: now,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("your_secret_key".as_ref())
    ).unwrap();

    web::Json(json!({ "access_token": token }))
}

async fn hello() -> impl Responder {
    "Hello, OIDC!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting OIDC server...");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
            .route("/token", web::post().to(token_endpoint))
    })
    .bind("localhost:3000")?
    .run()
    .await
}
