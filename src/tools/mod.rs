pub(crate) mod add;
pub(crate) mod divide;
pub(crate) mod multiply;

use serde::Deserialize;
use serde_json::{Value, json};

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
pub(crate) struct OperationArgs {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub(crate) enum ArithmeticError {
    #[error("invalid division: denominator must not be zero")]
    DivisionByZero,
    #[error("integer overflow while performing {operation}")]
    Overflow { operation: &'static str },
}

pub(crate) fn operation_parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "x": {
                "type": "integer",
                "minimum": i32::MIN,
                "maximum": i32::MAX,
                "description": "The first integer operand"
            },
            "y": {
                "type": "integer",
                "minimum": i32::MIN,
                "maximum": i32::MAX,
                "description": "The second integer operand"
            }
        },
        "required": ["x", "y"]
    })
}

pub(crate) fn add(args: OperationArgs) -> Result<i32, ArithmeticError> {
    args.x.checked_add(args.y).ok_or(ArithmeticError::Overflow {
        operation: "addition",
    })
}

pub(crate) fn multiply(args: OperationArgs) -> Result<i32, ArithmeticError> {
    args.x.checked_mul(args.y).ok_or(ArithmeticError::Overflow {
        operation: "multiplication",
    })
}

pub(crate) fn divide(args: OperationArgs) -> Result<i32, ArithmeticError> {
    if args.y == 0 {
        return Err(ArithmeticError::DivisionByZero);
    }

    args.x.checked_div(args.y).ok_or(ArithmeticError::Overflow {
        operation: "division",
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arithmetic_helpers_return_expected_results() {
        assert_eq!(add(OperationArgs { x: 8, y: 4 }), Ok(12));
        assert_eq!(add(OperationArgs { x: -8, y: 8 }), Ok(0));
        assert_eq!(multiply(OperationArgs { x: -3, y: 4 }), Ok(-12));
        assert_eq!(multiply(OperationArgs { x: 0, y: 9 }), Ok(0));
        assert_eq!(divide(OperationArgs { x: 18, y: 3 }), Ok(6));
        assert_eq!(divide(OperationArgs { x: -9, y: 3 }), Ok(-3));
    }

    #[test]
    fn division_by_zero_returns_an_explicit_error() {
        let result = divide(OperationArgs { x: 10, y: 0 });

        assert_eq!(result, Err(ArithmeticError::DivisionByZero));
        assert_eq!(
            ArithmeticError::DivisionByZero.to_string(),
            "invalid division: denominator must not be zero"
        );
    }

    #[test]
    fn arithmetic_overflow_returns_typed_errors() {
        assert_eq!(
            add(OperationArgs { x: i32::MAX, y: 1 }),
            Err(ArithmeticError::Overflow {
                operation: "addition"
            })
        );
        assert_eq!(
            multiply(OperationArgs { x: i32::MAX, y: 2 }),
            Err(ArithmeticError::Overflow {
                operation: "multiplication"
            })
        );
        assert_eq!(
            divide(OperationArgs { x: i32::MIN, y: -1 }),
            Err(ArithmeticError::Overflow {
                operation: "division"
            })
        );
    }

    #[test]
    fn operation_args_reject_invalid_json() {
        assert!(serde_json::from_str::<OperationArgs>(r#"{"x": 1}"#).is_err());
        assert!(serde_json::from_str::<OperationArgs>(r#"{"x": 1, "y": 1.5}"#).is_err());
        assert!(serde_json::from_str::<OperationArgs>(r#"{"x": "1", "y": 2}"#).is_err());
    }

    #[test]
    fn shared_schema_requires_two_integer_operands() {
        let schema = operation_parameters();

        assert_eq!(schema["type"], "object");
        assert_eq!(schema["properties"]["x"]["type"], "integer");
        assert_eq!(schema["properties"]["y"]["type"], "integer");
        assert_eq!(schema["properties"]["x"]["minimum"], i32::MIN);
        assert_eq!(schema["properties"]["x"]["maximum"], i32::MAX);
        assert_eq!(schema["properties"]["y"]["minimum"], i32::MIN);
        assert_eq!(schema["properties"]["y"]["maximum"], i32::MAX);
        assert_eq!(schema["required"], json!(["x", "y"]));
    }
}
