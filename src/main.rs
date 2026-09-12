mod shared;

// use crate::shared::file_system;

// const DIRECTORY_PATH: &str = "C:\\oper\\me\\ministack-microservices";

// fn main() {
//     let files: Vec<String> = file_system::list_files_in_directory(DIRECTORY_PATH);
//     print!("{:?}", files);
// }






use anyhow::Result;
use rig::prelude::*;
use rig::providers::openrouter;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenvy::dotenv().ok();
    
    let client = openrouter::Client::from_env()?;

    let agent = client
        .agent("nvidia/nemotron-3-ultra-550b-a55b:free")
        .preamble("You are a helpful assistant.")
        .build();

    // Send a prompt and await the model's reply.
    let response = agent.prompt("What is the Rust programming language?").await?;

    println!("{response}");

    Ok(())
}