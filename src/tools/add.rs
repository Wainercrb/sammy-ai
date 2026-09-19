use crate::tools::{ArithmeticError, OperationArgs, add, operation_parameters};
use rig::tool::Tool;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub(crate) struct AddOperation;

impl Tool for AddOperation {
    const NAME: &'static str = "add";

    type Error = ArithmeticError;
    type Args = OperationArgs;
    type Output = i32;

    fn description(&self) -> String {
        String::from("Add two integers")
    }

    fn parameters(&self) -> serde_json::Value {
        operation_parameters()
    }

    async fn call(
        &self,
        _context: &mut rig::tool::ToolContext,
        args: Self::Args,
    ) -> Result<Self::Output, Self::Error> {
        add(args)
    }
}
