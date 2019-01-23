extern crate authorization_service;

use std::error::Error;
use lambda_runtime::{error::HandlerError, lambda, Context};
use log::{self, error};
use serde_derive::{Deserialize, Serialize};
use simple_logger;
use authorization_service::execute;


#[derive(Deserialize)]
struct CustomEvent {
    #[serde(rename = "query")]
    query: Option<String>,
}

#[derive(Serialize)]
struct CustomOutput {
    message: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    lambda!(my_handler);

    Ok(())
}

fn my_handler(e: CustomEvent, c: Context) -> Result<CustomOutput, HandlerError> {

    match e.query {
        None => {
        error!("Query is requred {}", c.aws_request_id);
            return Err(c.new_error("Missing Query"))
        }
        Some(query) => {
            execute(&query[..]);
            Ok(CustomOutput {
                message: format!("Ok {:?}", query),
            })
        }
    }
}
