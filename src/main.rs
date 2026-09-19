mod shared;
mod tools;
// use crate::shared::file_system;

// const DIRECTORY_PATH: &str = "C:\\oper\\me\\ministack-microservices";

// fn main() {
//     let files: Vec<String> = file_system::list_files_in_directory(DIRECTORY_PATH);
//     print!("{:?}", files);
// }

use anyhow::Result;
use rig::prelude::*;
use rig::providers::openrouter;
use std::io::{self, Write};

pub use tools::add::AddOperation;
pub use tools::multiply::MultiplyOperation;
pub use tools::divide::DivideOperation;


#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenvy::dotenv().ok();

    let client = openrouter::Client::from_env()?;

    // let agent = client
    //     .agent("nvidia/nemotron-3-ultra-550b-a55b:free")
    //     .preamble("You are a helpful assistant.")
    //     .build();

    // // Send a prompt and await the model's reply.
    // let response = agent.prompt("What is the Rust programming language?").await?;

    // println!("{response}");

    // Ok(())

    // let agent = client
    //     .agent("nvidia/nemotron-3-ultra-550b-a55b:free")
    //     .preamble("You are a helpful assistant.")
    //     .build();

    let agent = client
        .agent("nvidia/nemotron-3-ultra-550b-a55b:free")
        .preamble("You are a calculator here to help the user perform arithmetic operations. Use the tools provided to answer the user questions.")
        .max_tokens(1024)
        .default_max_turns(2)
        .tool(AddOperation)
        .tool(MultiplyOperation)
        .tool(DivideOperation)
        .build();

    let mut input = String::new();

    println!("Please enter some text!");

    io::stdout().flush().expect("Failed to flush stdout");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the line");

    let result = agent.prompt(input).await?;

    println!("{}", result);

    Ok(())
}
