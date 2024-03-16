// src/middleware.rs

use pavex::middleware::Next;
use pavex::response::Response;
use std::future::IntoFuture;

pub async fn logging_middleware<C>(next: Next<C>) -> Response
    where
        C: IntoFuture<Output=Response>,
{
    println!("Before handling the request");
    let response = next.await;
    println!("After handling the request");
    response
}
