use actix_web::{get, web, App, HttpServer, Responder, Result};

use shared::models::Tweet;

#[get("/tweets")]
pub async fn get_tweets() -> Result<impl Responder> {
    let tweets = vec! {
        Tweet {
            content: "Hello!".to_string(),
        },
        Tweet {
            content: "World!".to_string(),
        }
    };

    Ok(web::Json(tweets))
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_tweets))
        .bind(("127.0.0.1", 9000))?
        .run()
        .await
}
