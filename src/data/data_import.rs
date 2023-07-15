use bson::{doc, Bson};
use dotenv::dotenv;
use std::fs;

use crate::models::practice_models::{mongodb, Organization};

pub async fn import() {
    dotenv().ok();
    // Read the files to memory
    let lead = fs::read_to_string("data/lead.json").expect("Unable to read file");
    let line = fs::read_to_string("data/line.json").expect("Unable to read file");
    let team = fs::read_to_string("data/team.json").expect("Unable to read file");
    // Deserialise the data so we can work with it
    let lead_organization: Organization = serde_json::from_str(&lead).unwrap();
    let line_organization: Organization = serde_json::from_str(&line).unwrap();
    let team_organization: Organization = serde_json::from_str(&team).unwrap();

    // Connect to mongodb
    let db = mongodb().await;

    // Write the Practice Leads to the DB if its empty
    for (group, people) in &lead_organization {
        let collection = db.collection(group);
        if collection
            .count_documents(None, None)
            .await
            .expect("Cannot access database")
            == 0
        {
            println!("importing {} from json:", group);
            for person in people {
                let doc = doc! {
                    "name": &person.name,
                    "band": &person.band,
                    "manager": &person.manager,
                    "role": &person.role,
                    "skills": &person.skills.iter().map(|s| Bson::String(s.clone())).collect::<Vec<Bson>>(),
                    "certificates": &person.certificates.iter().map(|c| Bson::String(c.clone())).collect::<Vec<Bson>>(),
                    "office": &person.office,
                    "location": &person.location
                };
                collection
                    .insert_one(doc, None)
                    .await
                    .expect("Failed to add group");
            }
        } else {
            println!("Data already present for {}, not importing", group)
        }
    }

    // Write the Line Managers to the DB
    for (group, people) in &line_organization {
        let collection = db.collection(group);
        if collection
            .count_documents(None, None)
            .await
            .expect("Cannot access database")
            == 0
        {
            println!("importing {} from json:", group);
            for person in people {
                let doc = doc! {
                    "name": &person.name,
                    "band": &person.band,
                    "manager": &person.manager,
                    "role": &person.role,
                    "skills": &person.skills.iter().map(|s| Bson::String(s.clone())).collect::<Vec<Bson>>(),
                    "certificates": &person.certificates.iter().map(|c| Bson::String(c.clone())).collect::<Vec<Bson>>(),
                    "office": &person.office,
                    "location": &person.location
                };
                collection
                    .insert_one(doc, None)
                    .await
                    .expect("Failed to add group");
            }
        } else {
            println!("Data already present for {}, not importing", group)
        }
    }

    // Write the Team Members to the DB
    for (group, people) in &team_organization {
        let collection = db.collection(group);
        if collection
            .count_documents(None, None)
            .await
            .expect("Cannot access database")
            == 0
        {
            println!("importing {} from json:", group);
            for person in people {
                let doc = doc! {
                    "name": &person.name,
                    "band": &person.band,
                    "manager": &person.manager,
                    "role": &person.role,
                    "skills": &person.skills.iter().map(|s| Bson::String(s.clone())).collect::<Vec<Bson>>(),
                    "certificates": &person.certificates.iter().map(|c| Bson::String(c.clone())).collect::<Vec<Bson>>(),
                    "office": &person.office,
                    "location": &person.location
                };
                collection
                    .insert_one(doc, None)
                    .await
                    .expect("Failed to add group");
            }
        } else {
            println!("Data already present for {}, not importing", group)
        }
    }
}
