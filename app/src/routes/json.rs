// src/handlers.rs

use pavex::http::{StatusCode, HeaderValue};
use pavex::request::body::JsonBody;
use pavex::response::Response;
use serde::{Serialize, Deserialize};
use serde_json::to_string;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}

pub fn create_user(body: JsonBody<User>) -> Response {
    let user = body.0;
    match to_string(&user) {
        Ok(json_str) => {
            let mut response = Response::new(StatusCode::OK);
            response.headers_mut().insert("Content-Type", HeaderValue::from_static("application/json"));
            response.body(); // Correctly setting the body here
            response
        },
        Err(_) => Response::new(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub fn get_user() -> Response {
    let user = User {
        id: 1,
        name: "Jane Doe".to_string(),
        email: "jane.doe@example.com".to_string(),
    };

    match to_string(&user) {
        Ok(json_str) => {
            let mut response = Response::new(StatusCode::OK);
            response.headers_mut().insert("Content-Type", HeaderValue::from_static("application/json"));
            response.body(); // Correctly setting the body here
            response
        },
        Err(_) => Response::new(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
