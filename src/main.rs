mod tools;

use anyhow::Result;
use rig::prelude::*;
use rig::providers::openrouter;
use std::io::{self, Write};

use crate::tools::add::AddOperation;
use crate::tools::divide::DivideOperation;
use crate::tools::multiply::MultiplyOperation;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenvy::dotenv().ok();

    let client = openrouter::Client::from_env()?;

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

    println!("Enter a calculation request:");

    io::stdout().flush()?;

    io::stdin().read_line(&mut input)?;

    let result = agent.prompt(input).await?;

    println!("{}", result);

    Ok(())
}
