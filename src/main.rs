extern crate authorization_service;


use authorization_service::protocol::Query;

fn main() {
    let q = Query {};
    println!("{:?}",q);
    println!("Hello, world!");
}
