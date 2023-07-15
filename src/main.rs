mod data;
mod models;

use std::{fs::File, io::Write};

use data::{data_import, diagrams};

#[tokio::main]
async fn main() {
    data_import::import().await;
    let diagram = diagrams::draw_diagrams().await;
    let mut file = File::create("diagram/diagram.dot").expect("Failed to create file");
    file.write_all(diagram.as_bytes())
        .expect("Failed to write to file");
}
