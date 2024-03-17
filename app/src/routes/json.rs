use pavex::http::{StatusCode, HeaderValue};
use pavex::request::body::JsonBody;
use pavex::response::Response;
use serde::{Serialize, Deserialize};
use serde_json::{to_string};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}

// Assuming `Response` has a method to set the body directly or through constructing a new instance
pub fn create_user(body: JsonBody<User>) -> Response {
    let user = body.0;
    let saved_user = User { id: 1, ..user };

    // Serialize the user object into a JSON string
    match to_string(&saved_user) {
        Ok(_json_str) => {
            let mut response = Response::new(StatusCode::OK);
            // Set headers and body assuming `Response` API allows for this pattern
            response.headers_mut().insert("Content-Type", HeaderValue::from_static("application/json"));
            response.body();
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

    // Serialize the user struct to a JSON string
    match to_string(&user) {
        Ok(json) => json,
        Err(_) => return Response::new(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let mut response = Response::new(StatusCode::OK);
    response.headers_mut().insert("Content-Type", HeaderValue::from_static("application/json"));
    response.body();
    response
}


// pub fn get_user() -> Response {
//     let user = User {
//         id: 1,
//         name: "Jane Doe".to_string(),
//         email: "jane.doe@example.com".to_string(),
//     };
//     match to_string(&user) {
//         Ok(_json_str) => {
//             let mut response = Response::new(StatusCode::OK);
//             response.headers_mut().insert("Content-Type", HeaderValue::from_static("application/json"));
//             response.body();
//             response
//         },
//         Err(_) => Response::new(StatusCode::INTERNAL_SERVER_ERROR),
//     }
// }
