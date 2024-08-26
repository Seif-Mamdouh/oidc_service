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

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App};
    use jsonwebtoken::{decode, DecodingKey, Validation};

    #[actix_rt::test]
    async fn test_token_endpoint() {
        let app = App::new().route("/token", web::post().to(token_endpoint));
        let mut app = test::init_service(app).await;

        let req = test::TestRequest::post().uri("/token").to_request();
        let resp: serde_json::Value = test::call_and_read_body_json(&mut app, req).await;

        assert!(resp.get("access_token").is_some());

        let token = resp["access_token"].as_str().unwrap();
        let decoded = decode::<Claims>(
            token,
            &DecodingKey::from_secret("your_secret_key".as_ref()),
            &Validation::default()
        ).unwrap();

        assert_eq!(decoded.claims.sub, "github-action");
    }
}
