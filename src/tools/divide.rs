use rig::tool::Tool;
use serde::{Deserialize, Serialize};
use serde_json::json;
use anyhow::Result;

#[derive(Deserialize)]
pub struct OperationArgs {
    x: i32,
    y: i32,
}

#[derive(Debug, thiserror::Error)]
#[error("Match error")]
pub struct MatchError;

#[derive(Deserialize, Serialize)]
pub struct DivideOperation;

impl Tool for DivideOperation {
    const NAME: &'static str = "divide";

    type Error = MatchError;
    type Args = OperationArgs;
    type Output = i32;

    fn description(&self) -> String {
        String::from("Divide x and y together")
    }

    fn parameters(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "x": {
                    "type": "number",
                    "description": "The first number to add"
                },
                 "y": {
                    "type": "number",
                    "description": "The second number to add"
                },
            },
            "required": ["x", "y"]
        })
    }

    async fn call(
        &self,
        _context: &mut rig::tool::ToolContext,
        args: Self::Args,
    ) -> Result<Self::Output, Self::Error> {
        println!("[tool-call] divide {} and {}", args.x, args.y);
        let result = args.x / args.y;
        Ok(result)
    }
}
