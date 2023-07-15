use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client, Database};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Person {
    pub name: String,
    pub band: String,
    pub manager: String,
    pub role: String,
    pub skills: Vec<String>,
    pub certificates: Vec<String>,
    pub office: String,
    pub location: String,
}

pub type Organization = HashMap<String, Vec<Person>>;

pub async fn mongodb() -> Database {
    dotenv().ok();
    let uri = match env::var("MONGODB_URI") {
        Ok(v) => v,
        Err(_) => format!("Error loading env variable"),
    };
    let client_options = ClientOptions::parse(uri)
        .await
        .expect("Failed to parse connection string");
    let client = Client::with_options(client_options).expect("Failed to connect to db");
    // Set DB, this will be created if it doesn't already exist
    client.database("practice")
}
