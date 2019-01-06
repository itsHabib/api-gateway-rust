#[macro_use]
extern crate serde_derive;
extern crate serde_json;
use std::error::Error;

use http::{Error as HttpError, StatusCode};
use lambda_http::{lambda, Body, IntoResponse, Request, RequestExt, Response};
use lambda_runtime::{error::HandlerError, Context};
use log::{self, error};
use simple_logger;

#[derive(Deserialize, Serialize, Debug)]
struct User {
    username: String,
    email: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    lambda!(|req, c| router(req, c));
    Ok(())
}

fn router(req: Request, c: Context) -> Result<impl IntoResponse, HandlerError> {
    match req.method().as_str() {
        "GET" => get_users(req, c),
        "POST" => add_user(req, c),
        _ => not_allowed(req, c),
    }
}

fn not_allowed(req: Request, c: Context) -> Result<Response<Body>, HandlerError> {
    Response::builder()
        .status(StatusCode::METHOD_NOT_ALLOWED)
        .body(Body::from(()))
        .map_err(|err| http_to_handler_err(err, c))
}

fn get_users(req: Request, c: Context) -> Result<Response<Body>, HandlerError> {
    /*
     *getting path parameters
     *let path_params = req.path_parameters();
     *let username: Option<&str> = path_params.get("username");
     */
    Ok(serde_json::json!(init_users()).into_response())
}

fn add_user(req: Request, c: Context) -> Result<Response<Body>, HandlerError> {
    match serde_json::from_slice::<User>(req.body().as_ref()) {
        Ok(user) => {
            let mut resp = serde_json::json!(user).into_response();
            *resp.status_mut() = StatusCode::CREATED;
            Ok(resp)
        }
        Err(_) => Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body("bad request".into())
            .map_err(|err| http_to_handler_err(err, c)),
    }
}

fn http_to_handler_err(err: HttpError, c: Context) -> HandlerError {
    c.new_error(&*format!("{}", err))
}

fn init_users() -> Vec<User> {
    ["itshabib", "painter11", "derk90", "coder12"]
        .iter()
        // needed to dereference
        .map(|name_ref| fill_user_fields(*name_ref))
        .collect()
}

fn fill_user_fields(name: &str) -> User {
    User {
        username: name.to_string(),
        email: format!("{}@email.com", name),
    }
}