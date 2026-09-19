use crate::tools::{ArithmeticError, OperationArgs, divide, operation_parameters};
use rig::tool::Tool;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub(crate) struct DivideOperation;

impl Tool for DivideOperation {
    const NAME: &'static str = "divide";

    type Error = ArithmeticError;
    type Args = OperationArgs;
    type Output = i32;

    fn description(&self) -> String {
        String::from("Divide one integer by another")
    }

    fn parameters(&self) -> serde_json::Value {
        operation_parameters()
    }

    async fn call(
        &self,
        _context: &mut rig::tool::ToolContext,
        args: Self::Args,
    ) -> Result<Self::Output, Self::Error> {
        divide(args)
    }
}
