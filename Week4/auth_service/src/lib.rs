#![allow(dead_code, unused_variables)]
use rand::prelude::*;
mod database;
mod auth_utils;

//using pub, we are brigning Credentials into export
pub use auth_utils::models::Credentials;
use database::Status;

pub fn authenticate(creds: Credentials) {
    let timeout = 
        thread_rng().gen_range(100..500);
    println!("The timeout is {timeout}");
    if let Status::Connected = database::conenct_to_database(){
        auth_utils::login(creds);
    }
}
