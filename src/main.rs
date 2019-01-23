extern crate authorization_service;

use std::error::Error;
use lambda_runtime::{error::HandlerError, lambda, Context};
use log::{self, error};
use serde_derive::{Deserialize};
use simple_logger;
use authorization_service::execute_r;
use authorization_service::XRet;


#[derive(Deserialize)]
struct CustomEvent {
    #[serde(rename = "query")]
    query: Option<String>,
}

// #[derive(Serialize)]
// struct CustomOutput {
//     message: String,
// }

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    lambda!(my_handler_2);
    Ok(())
}

fn my_handler_2(e: CustomEvent, c: Context) -> Result<XRet, HandlerError> {

    match e.query {
        None => {
        error!("Query is requred {}", c.aws_request_id);
            return Err(c.new_error("Missing Query"))
        }
        Some(query) => {
            let r = execute_r(&query[..]);
            Ok(r)
        }
    }
}
