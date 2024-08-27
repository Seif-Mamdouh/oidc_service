use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use reqwest;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::RwLock;
use color_eyre::eyre::{Result, eyre};

#[derive(Debug, Serialize, Deserialize)]
struct GitHubClaims {
    sub: String,
    repository: String,
    repository_owner: String,
    job_workflow_ref: String,
    iat: u64,
}

struct AppState {
    jwks: Arc<RwLock<Value>>,
}

#[derive(Deserialize)]
struct TokenRequest {
    token: String,
}

async fn token_endpoint(
    token_request: web::Json<TokenRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    match validate_github_token(&token_request.token, data.jwks.clone()).await {
        Ok(claims) => HttpResponse::Ok().json(claims),
        Err(e) => {
            eprintln!("Token validation error: {:?}", e);
            HttpResponse::BadRequest().body(format!("Invalid token: {}", e))
        }
    }
}

async fn validate_github_token(
    token: &str,
    jwks: Arc<RwLock<Value>>,
) -> Result<GitHubClaims> {
    if !token.starts_with("eyJ") {
        return Err(eyre!("Invalid token format. Expected a JWT."));
    }

    let jwks = jwks.read().await;
    
    let header = jsonwebtoken::decode_header(token)
        .map_err(|e| eyre!("Failed to decode header: {}. Make sure you're using a valid JWT, not a PAT.", e))?;
    
    let decoding_key = if let Some(kid) = header.kid {
        let key = jwks["keys"]
            .as_array()
            .ok_or_else(|| eyre!("Invalid JWKS format"))?
            .iter()
            .find(|k| k["kid"].as_str() == Some(&kid))
            .ok_or_else(|| eyre!("Matching key not found in JWKS"))?;

        let modulus = key["n"].as_str().ok_or_else(|| eyre!("No 'n' in JWK"))?;
        let exponent = key["e"].as_str().ok_or_else(|| eyre!("No 'e' in JWK"))?;

        DecodingKey::from_rsa_components(modulus, exponent)
            .map_err(|e| eyre!("Failed to create decoding key: {}", e))?
    } else {
        DecodingKey::from_secret("your_secret_key".as_ref())
    };

    let mut validation = Validation::new(Algorithm::RS256);
    
    validation.set_audience(&["https://github.com/Seif-Mamdouh"]);

    let token_data = decode::<GitHubClaims>(token, &decoding_key, &validation)
        .map_err(|e| eyre!("Failed to decode token: {}", e))?;

    let claims = token_data.claims;

    
    if let Ok(org) = std::env::var("GITHUB_ORG") {
        if claims.repository_owner != org {
            return Err(eyre!("Token is not from the expected organization"));
        }
    }

    
    if let Ok(repo) = std::env::var("GITHUB_REPO") {
        if claims.repository != repo {
            return Err(eyre!("Token is not from the expected repository"));
        }
    }

    Ok(claims)
}

async fn fetch_jwks(oidc_url: &str) -> Result<Value> {
    let client = reqwest::Client::new();
    let jwks_url = format!("{}/.well-known/jwks", oidc_url);
    let jwks = client.get(&jwks_url).send().await?.json().await?;
    Ok(jwks)
}

async fn hello() -> impl Responder {
    "Hello, OIDC!"
}

#[actix_web::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let github_oidc_url = "https://token.actions.githubusercontent.com";
    let jwks = Arc::new(RwLock::new(
        fetch_jwks(github_oidc_url).await?
    ));

    if let Ok(org) = std::env::var("GITHUB_ORG") {
        println!("GITHUB_ORG set to: {}", org);
    }
    if let Ok(repo) = std::env::var("GITHUB_REPO") {
        println!("GITHUB_REPO set to: {}", repo);
    }

    println!("Starting OIDC server...");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                jwks: jwks.clone(),
            }))
            .route("/", web::get().to(hello))
            .route("/token", web::post().to(token_endpoint))
    })
    .bind("localhost:3000")?
    .run()
    .await?;

    Ok(())
}
