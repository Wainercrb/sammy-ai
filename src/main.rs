mod tools;

use anyhow::Result;
use rig::prelude::*;
use rig::providers::openrouter;
use rig::completion::Message; //  ¡Corregido!
use std::io::{self, Write};

use crate::tools::code_fixer_tool::CodeFixerTool;
use crate::tools::path_checker_tool::PathCheckerTool;
use crate::tools::project_checker_tool::ProjectEvaluatorTool;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenvy::dotenv().ok();

    let client = openrouter::Client::from_env()?;

    let agent = client
        .agent("nvidia/nemotron-3-ultra-550b-a55b:free")
        .preamble(
            "You are a stateful code orchestration terminal. Ensure you capture inputs step-by-step:
            
            Strict Execution Rules:
            1. STEP 1 (Turn 1): User provides a path string. You MUST invoke 'path_checker'. Output folder status details and ask: 'What check options or targets should we evaluate?'
            2. STEP 2 (Turn 2): User provides custom flags/options. You MUST call 'project_evaluator' providing both the remembered path and this input text. Present logs back, then ask: 'What final action or report rule should I run?'
            3. STEP 3 (Turn 3): User inputs final actions. Call 'code_fixer' combining path, previous options, and this final instruction. Display completion summary."
        )
        .max_tokens(1024)
        .default_max_turns(7)
        .tool(PathCheckerTool)
        .tool(ProjectEvaluatorTool)
        .tool(CodeFixerTool)
        .build();

    // 👈 Rig 0.42.0: Inicializamos el vector dinámico que almacenará la memoria de la terminal
    let mut chat_history: Vec<Message> = vec![];

    println!("===============================================================");
    println!("🤖 RUNNING LIVE MULTI-STEP CODE EVALUATOR INTERACTIVE SHELL");
    println!("===============================================================");
    println!("👉 STEP 1: Provide a valid local system directory path to analyze:");

    loop {
        print!("code-terminal> ");
        io::stdout().flush()?;

        let mut incoming = String::new();
        io::stdin().read_line(&mut incoming)?;
        let trimmed = incoming.trim();

        if trimmed.eq_ignore_ascii_case("exit") || trimmed.eq_ignore_ascii_case("quit") {
            println!("Terminating terminal sequence. See you!");
            break;
        }

        if trimmed.is_empty() {
            continue;
        }

        println!("[system]: Contacting agent core layers...");
        
        // 👈 Rig 0.42.0 syntax: Llamamos a .chat() pasándole el prompt y la referencia mutable de la historia
        match agent.chat(trimmed, &mut chat_history).await {
            Ok(agent_speech) => {
                // El agente añade automáticamente el prompt del usuario y su respuesta al vector
                println!("\nagent> {}\n", agent_speech);
            }
            Err(runtime_fault) => {
                println!("\n[Agent Engine Error]: {}\n", runtime_fault);
            }
        }
    }

    Ok(())
}
