use rig::tool::Tool;
use serde::{Deserialize, Serialize};
use serde_json::json;
use schemars::JsonSchema;
use std::path::Path;
use std::process::Command;

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct EvaluatorArgs {
    #[schemars(description = "The local folder path of the project remembered from Step 1")]
    pub path: String,
    #[schemars(description = "The compilation or analysis options provided manually by the user in Step 2")]
    pub user_option: String,
}

#[derive(Debug, thiserror::Error)]
#[error("Project evaluator error")]
pub struct EvaluatorError;

#[derive(Deserialize, Serialize)]
pub struct ProjectEvaluatorTool;

struct ProjectEcosystem {
    name: &'static str,
    manifest_file: &'static str,
    base_command: &'static str,
    default_args: Vec<&'static str>,
}

impl Tool for ProjectEvaluatorTool {
    const NAME: &'static str = "project_evaluator";
    type Error = EvaluatorError;
    type Args = EvaluatorArgs;
    type Output = String;

    fn description(&self) -> String {
        String::from("Detects the project language ecosystem (Rust, Node.js, Python) and runs its default evaluation appending your custom flags.")
    }

    fn parameters(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "path": { "type": "string", "description": "Target project directory path" },
                "user_option": { "type": "string", "description": "Custom user arguments or targets (e.g. '--release', 'test')" }
            },
            "required": ["path", "user_option"]
        })
    }

    async fn call(&self, _context: &mut rig::tool::ToolContext, args: Self::Args) -> Result<Self::Output, Self::Error> {
        println!("\n[tool-invoked]: project_evaluator");
        println!(" -> Path: {}", args.path);
        println!(" -> Options: {}", args.user_option);

        let target_path = Path::new(&args.path);

        let ecosystems = vec![
            ProjectEcosystem {
                name: "Rust",
                manifest_file: "Cargo.toml",
                base_command: "cargo",
                default_args: vec!["check", "--message-format=short"],
            },
            ProjectEcosystem {
                name: "Node.js",
                manifest_file: "package.json",
                base_command: "npm",
                default_args: vec!["run"],
            },
            ProjectEcosystem {
                name: "Python",
                manifest_file: "requirements.txt",
                base_command: "python",
                default_args: vec!["-m", "compileall"],
            },
        ];

        let mut detected = None;
        for eco in ecosystems {
            if target_path.join(eco.manifest_file).exists() {
                detected = Some(eco);
                break;
            }
        }

        let Some(eco) = detected else {
            return Ok(format!("Validation Failed: No standard configuration manifest discovered in '{}'. Cannot evaluate.", args.path));
        };

        let mut final_args: Vec<String> = eco.default_args.iter().map(|s| s.to_string()).collect();
        
        if !args.user_option.is_empty() && args.user_option.to_lowercase() != "none" {
            for word in args.user_option.split_whitespace() {
                final_args.push(word.to_string());
            }
        }

        println!(" -> Executing: {} {:?}", eco.base_command, final_args);

        let output = Command::new(eco.base_command)
            .args(&final_args)
            .current_dir(target_path)
            .output();

        match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                let stderr = String::from_utf8_lossy(&out.stderr).to_string();
                let mut report = format!("Ecosystem identified: {} project.\n", eco.name);

                if out.status.success() {
                    report.push_str("Success: The evaluation command completed successfully!\n");
                } else {
                    report.push_str("Status: The evaluation command flagged errors/warnings.\n");
                }
                report.push_str(&format!("Terminal Outputs:\n{}{}", stdout, stderr));
                Ok(report)
            }
            Err(_) => Ok(format!("Error: Found {} manifest, but CLI runtime '{}' is missing on this machine.", eco.name, eco.base_command)),
        }
    }
}
