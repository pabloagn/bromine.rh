use anyhow::{Result, Context};
use tokio::process::Command;

pub async fn execute_command(command: &str) -> Result<String> {
    let output = Command::new("bash")
        .arg("-c")
        .arg(command)
        .output()
        .await
        .context("Failed to execute bash command")?;
    
    if !output.status.success() {
        anyhow::bail!(
            "Command failed with status: {}\nStderr: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        );
    }
    
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub async fn execute_menu_command(command: &str) -> Result<String> {
    // Execute command and expect JSON output
    execute_command(command).await
}
