#[macro_use]
extern crate dynomite;
#[macro_use]
extern crate dynomite_derive;
extern crate rand;
extern crate rusoto_core;
extern crate rusoto_dynamodb;

use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, PutItemInput};

#[derive(Item, Debug, Clone)]
struct User {
    username: String,
    email: String,
}

fn main() {
    let usernames = vec![
        String::from("jim93"),
        String::from("itshabib"),
        String::from("painter11"),
        String::from("slayermh"),
    ];
    let client = DynamoDbClient::new(Region::UsEast1);
    let table_name = String::from("users");
    for username in usernames.iter() {
        let put_input = PutItemInput {
            table_name: table_name.to_string(),
            item: User {
                username: username.to_string().clone(),
                email: format!("{}@example.com", username),
            }
            .into(),
            ..PutItemInput::default()
        };

        client.put_item(put_input).sync().unwrap();
    }
}
