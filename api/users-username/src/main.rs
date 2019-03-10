#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

use http::StatusCode;
use lambda_http::{lambda, Body, IntoResponse, Request, RequestExt, Response};
use lambda_runtime::{error::HandlerError, Context};
use rusoto_core::Region;
use rusoto_dynamodb::{
    AttributeValue, DeleteItemError, DeleteItemInput, DynamoDb, DynamoDbClient, GetItemError,
    GetItemInput, UpdateItemError, UpdateItemInput,
};
use std::collections::HashMap;
use std::error::Error;

#[derive(Deserialize, Serialize, Debug, Clone)]
struct User {
    username: String,
    email: String,
    age: i32,
}

impl User {
    fn get_update_expression() -> String {
        "SET email = :email, age = :age".to_owned()
    }

    fn get_expression_attr_values(&self) -> HashMap<String, AttributeValue> {
        self.to_attr_map()
            .iter_mut()
            .map(|(k, v)| (format!(":{}", k), v.to_owned()))
            .collect()
    }
}

impl From<HashMap<String, AttributeValue>> for User {
    fn from(attr_map: HashMap<String, AttributeValue>) -> Self {
        let age = attr_map
            .get("age")
            .and_then(|v| v.n.clone())
            .unwrap_or_default();
        let username = attr_map
            .get("username")
            .and_then(|v| v.s.clone())
            .unwrap_or_default();
        let email = attr_map
            .get("email")
            .and_then(|v| v.s.clone())
            .unwrap_or_default();

        User {
            username,
            email,
            age: age.parse::<i32>().unwrap_or_else(|_| 0),
        }
    }
}
impl From<&HashMap<String, AttributeValue>> for User {
    fn from(attr_map: &HashMap<String, AttributeValue>) -> Self {
        let age = attr_map
            .get("age")
            .and_then(|v| v.n.clone())
            .unwrap_or_default();
        let username = attr_map
            .get("username")
            .and_then(|v| v.s.clone())
            .unwrap_or_default();
        let email = attr_map
            .get("email")
            .and_then(|v| v.s.clone())
            .unwrap_or_default();

        User {
            username,
            email,
            age: age.parse::<i32>().unwrap_or_else(|_| 0),
        }
    }
}

impl User {
    fn to_attr_map(&self) -> HashMap<String, AttributeValue> {
        let mut map = HashMap::new();
        map.insert(
            "username".to_string(),
            AttributeValue {
                s: Some(self.username.clone()),
                ..Default::default()
            },
        );
        map.insert(
            "email".to_string(),
            AttributeValue {
                s: Some(self.email.clone()),
                ..Default::default()
            },
        );
        map.insert(
            "age".to_string(),
            AttributeValue {
                n: Some(self.age.to_string().clone()),
                ..Default::default()
            },
        );
        map
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info)?;
    lambda!(router);
    Ok(())
}

// Call handler functionb based on request method
fn router(req: Request, c: Context) -> Result<impl IntoResponse, HandlerError> {
    match req.method().as_str() {
        "GET" => get_user(req, c),
        "PUT" | "POST" => update_user(req, c),
        "DELETE" => delete_user(req, c),
        _ => Ok(build_resp("".to_owned(), StatusCode::METHOD_NOT_ALLOWED)),
    }
}

fn get_user(req: Request, _c: Context) -> Result<Response<Body>, HandlerError> {
    // get username from path
    let path_params = req.path_parameters();
    match path_params.get("username") {
        Some(username) => {
            let mut key = HashMap::new();
            // key used to search table
            key.insert(
                "username".to_string(),
                AttributeValue {
                    s: Some(username.to_string().clone()),
                    ..Default::default()
                },
            );

            let client = DynamoDbClient::new(Region::UsEast1);
            let input = GetItemInput {
                table_name: "users".to_string(),
                key,
                ..Default::default()
            };
            // search table for user
            match client.get_item(input).sync() {
                Ok(output) => {
                    if let Some(item) = output.item {
                        let user: User = item.into();
                        Ok(serde_json::json!(user).into_response())
                    } else {
                        Ok(Response::default())
                    }
                }
                Err(e) => match e {
                    GetItemError::ResourceNotFound(_) => {
                        Ok(build_resp("not found".to_owned(), StatusCode::NOT_FOUND))
                    }
                    e => {
                        error!("{}", e);
                        Ok(build_resp(
                            format!("{}", e),
                            StatusCode::INTERNAL_SERVER_ERROR,
                        ))
                    }
                },
            }
        }
        None => Ok(build_resp(
            "bad request".to_owned(),
            StatusCode::BAD_REQUEST,
        )),
    }
}

fn delete_user(req: Request, _c: Context) -> Result<Response<Body>, HandlerError> {
    // get username from path
    let path_params = req.path_parameters();
    match path_params.get("username") {
        Some(username) => {
            let mut key = HashMap::new();
            key.insert(
                "username".to_string(),
                AttributeValue {
                    s: Some(username.to_string().clone()),
                    ..Default::default()
                },
            );
            let client = DynamoDbClient::new(Region::UsEast1);
            let input = DeleteItemInput {
                table_name: "users".to_string(),
                key,
                condition_expression: Some("attribute_exists(username)".to_string()),
                return_values: Some("ALL_OLD".to_string()),
                ..Default::default()
            };
            // deletet user
            match client.delete_item(input).sync() {
                Ok(output) => Ok(serde_json::json!(output.attributes).into_response()),
                Err(e) => match e {
                    DeleteItemError::ResourceNotFound(_)
                    | DeleteItemError::ConditionalCheckFailed(_) => {
                        Ok(build_resp("not found".to_owned(), StatusCode::NOT_FOUND))
                    }
                    e => {
                        error!("{}", e);
                        Ok(build_resp(
                            format!("{}", e),
                            StatusCode::INTERNAL_SERVER_ERROR,
                        ))
                    }
                },
            }
        }
        None => Ok(build_resp(
            "bad request".to_owned(),
            StatusCode::BAD_REQUEST,
        )),
    }
}

// PUT /users/{username}
fn update_user(req: Request, _c: Context) -> Result<Response<Body>, HandlerError> {
    // validate body
    match serde_json::from_slice::<User>(req.body().as_ref()) {
        Ok(user) => {
            // get username from path
            let path_params = req.path_parameters();
            match path_params.get("username") {
                Some(username) => {
                    let mut key = HashMap::new();
                    key.insert(
                        "username".to_string(),
                        AttributeValue {
                            s: Some(username.to_string().clone()),
                            ..Default::default()
                        },
                    );

                    let client = DynamoDbClient::new(Region::UsEast1);
                    let mut attr_values = user.get_expression_attr_values();
                    // remove username key
                    attr_values.remove(&(":username".to_owned()));
                    let input = UpdateItemInput {
                        table_name: "users".to_string(),
                        key,
                        condition_expression: Some("attribute_exists(username)".to_string()),
                        return_values: Some("ALL_NEW".to_string()),
                        update_expression: Some(User::get_update_expression()),
                        expression_attribute_values: Some(attr_values),
                        ..Default::default()
                    };
                    // update user
                    match client.update_item(input).sync() {
                        Ok(output) => {
                            if let Some(item) = output.attributes {
                                let user: User = item.into();
                                Ok(serde_json::json!(user).into_response())
                            } else {
                                Ok(build_resp(
                                    format!("{} not found", username),
                                    StatusCode::NOT_FOUND,
                                ))
                            }
                        }
                        Err(e) => match e {
                            UpdateItemError::ResourceNotFound(_)
                            | UpdateItemError::ConditionalCheckFailed(_) => {
                                Ok(build_resp("not found".to_owned(), StatusCode::NOT_FOUND))
                            }
                            e => {
                                error!("{}", e);
                                Ok(build_resp(
                                    "internal error".to_owned(),
                                    StatusCode::INTERNAL_SERVER_ERROR,
                                ))
                            }
                        },
                    }
                }
                None => Ok(build_resp(
                    "bad request".to_owned(),
                    StatusCode::BAD_REQUEST,
                )),
            }
        }

        Err(e) => {
            error!("{}", e);
            Ok(build_resp(
                format!("bad request: {}", e),
                StatusCode::BAD_REQUEST,
            ))
        }
    }
}

// simple response builder that uses a str msessage
fn build_resp(msg: String, status_code: StatusCode) -> Response<Body> {
    Response::builder()
        .status(status_code)
        .body(msg.into())
        .expect("err creating response")
}
