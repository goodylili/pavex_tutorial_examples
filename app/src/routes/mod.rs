// routes/mod.rs

pub mod status;
pub mod hello;
pub mod json;
pub mod middleware;

use pavex::blueprint::{router::GET, Blueprint};
use pavex::blueprint::router::POST;
use pavex::f;

pub fn register(bp: &mut Blueprint) {
    bp.wrap(f!(self::middleware::logging_middleware));
    bp.route(GET, "/api/ping", f!(self::status::ping));
    bp.route(GET, "/api/hello/:name", f!(self::hello::hello));
    bp.route(POST, "/api/users", f!(self::json::create_user));
    bp.route(GET, "/api/users/:id", f!(self::json::get_user));
}
