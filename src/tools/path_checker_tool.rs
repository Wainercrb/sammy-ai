use rig::tool::Tool;
use serde::{Deserialize, Serialize};
use serde_json::json;
use schemars::JsonSchema;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct OperationArgs {
    pub path: String,
}

#[derive(Debug, thiserror::Error)]
#[error("Path error")]
pub struct PathError;

#[derive(Deserialize, Serialize)]
pub struct PathCheckerTool;

impl Tool for PathCheckerTool {
    const NAME: &'static str = "path_checker";
    type Error = PathError;
    type Args = OperationArgs;
    type Output = String;

    fn description(&self) -> String {
        String::from("Check if given path exists and is a directory.")
    }

    fn parameters(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "path": { "type": "string", "description": "The path to check" }
            },
            "required": ["path"]
        })
    }

    async fn call(&self, _context: &mut rig::tool::ToolContext, args: Self::Args) -> Result<Self::Output, Self::Error> {
        println!("\n[tool-invoked]: path_checker for path: {}", args.path);
        let target = Path::new(&args.path);
        if !target.exists() {
            return Ok(format!("Error: the path '{}' does not exist.", args.path));
        }
        if !target.is_dir() {
            return Ok(format!("Error: the path '{}' is not a directory.", args.path));
        }
        Ok(format!("Success: The path '{}' is verified and open.", args.path))
    }
}
