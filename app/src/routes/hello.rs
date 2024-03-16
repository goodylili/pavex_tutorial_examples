use pavex::request::path::PathParams;
use pavex::response::Response;

#[PathParams]
pub struct HelloParameters {
    pub name: String,
}

pub fn hello(params: PathParams<HelloParameters>) -> Response {
    let HelloParameters { name } = params.0;
    Response::ok()
        .set_typed_body(format!("Hello, {name}!"))
}