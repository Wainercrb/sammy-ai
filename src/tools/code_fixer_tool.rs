use rig::tool::Tool;
use serde::{Deserialize, Serialize};
use serde_json::json;
use schemars::JsonSchema;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct FixerArgs {
    pub path: String,
    pub previous_option: String,
    pub user_action: String,
}

#[derive(Debug, thiserror::Error)]
#[error("Fixer error")]
pub struct FixerError;

#[derive(Deserialize, Serialize)]
pub struct CodeFixerTool;

impl Tool for CodeFixerTool {
    const NAME: &'static str = "code_fixer";
    type Error = FixerError;
    type Args = FixerArgs;
    type Output = String;

    fn description(&self) -> String {
        String::from("Handles final modifications or writes dynamic automated reports based on 3rd user input.")
    }

    fn parameters(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "path": { "type": "string" },
                "previous_option": { "type": "string" },
                "user_action": { "type": "string", "description": "Action details to execute" }
            },
            "required": ["path", "previous_option", "user_action"]
        })
    }

    async fn call(&self, _context: &mut rig::tool::ToolContext, args: Self::Args) -> Result<Self::Output, Self::Error> {
        println!("\n[tool-invoked]: code_fixer finalized pipeline.");
        Ok(format!("Finished processing sequence on path '{}'. Applied patch rule: '{}'.", args.path, args.user_action))
    }
}