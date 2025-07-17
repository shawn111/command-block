use std::future::Future;
use std::pin::Pin;
use tokio::process::Command;

pub fn run_command(cmd: &str) -> Pin<Box<dyn Future<Output = String> + Send>> {
    let cmd = cmd.to_string();
    Box::pin(async move {
        let output = Command::new("fish")
            .arg("-c")
            .arg(&cmd)
            .output()
            .await;

        match output {
            Ok(out) => {
                let mut result = String::from_utf8_lossy(&out.stdout).to_string();
                if !out.stderr.is_empty() {
                    result.push_str(&String::from_utf8_lossy(&out.stderr));
                }
                result
            }
            Err(e) => format!("Error: {}", e),
        }
    })
}
