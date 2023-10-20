use actix_web::{get, web, App, HttpServer, Responder};
use serde::Deserialize;

#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[derive(Deserialize)]
struct Numbers  {
    num1: i32,
    num2: i32,
    num3: Option<i32>
}

#[get("/add/")]
async fn addition(params: web::Query<Numbers>) -> impl Responder {
    let (num1, num2, num3) = (params.num1, params.num2, params.num3);
    match num3  {
        // Excuse the poorly formated code below for now, just playing around with enums & deserialization.
        Some(num3) => format!("{} + {} + {} = {}!", num1, num2, num3, add(add(num1, num2), num3)),
        None => format!("{} + {} = {}!", num1, num2, add(num1, num2))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(addition))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}