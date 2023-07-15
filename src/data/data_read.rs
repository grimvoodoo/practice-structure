use dotenv::dotenv;
use futures::stream::StreamExt;
use mongodb::bson::Document;

use crate::models::practice_models::{mongodb, Person};

pub async fn practice_leads() -> Vec<Person> {
    dotenv().ok();
    // Connect to mongodb
    let db = mongodb().await;

    // Read Practice Leads out of DB
    let lead_collection = db.collection::<Document>("practice_leads");
    let mut lead_cursor = lead_collection
        .find(None, None)
        .await
        .expect("Error reading DB");
    let mut lead: Vec<Person> = Vec::new();
    while let Some(result) = lead_cursor.next().await {
        match result {
            Ok(document) => {
                let person: Person = mongodb::bson::from_document(document).unwrap();
                lead.push(person);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    lead
}

pub async fn line_managers() -> Vec<Person> {
    dotenv().ok();
    // Connect to mongodb
    let db = mongodb().await;

    // Read Line Managers out of DB
    let line_collection = db.collection::<Document>("line_managers");
    let mut line_cursor = line_collection
        .find(None, None)
        .await
        .expect("Error reading DB");
    let mut line: Vec<Person> = Vec::new();
    while let Some(result) = line_cursor.next().await {
        match result {
            Ok(document) => {
                let person: Person = mongodb::bson::from_document(document).unwrap();
                line.push(person);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    line
}

pub async fn team_members() -> Vec<Person> {
    dotenv().ok();
    // Connect to mongodb
    let db = mongodb().await;

    // Read Team Members out of DB
    let team_collection = db.collection::<Document>("team");
    let mut team_cursor = team_collection
        .find(None, None)
        .await
        .expect("Error reading DB");
    let mut team: Vec<Person> = Vec::new();
    while let Some(result) = team_cursor.next().await {
        match result {
            Ok(document) => {
                let person: Person = mongodb::bson::from_document(document).unwrap();
                team.push(person);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    team
}
