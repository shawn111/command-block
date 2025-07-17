mod shell;
mod ui;

use crate::shell::run_command;
use crate::ui::ShellApp;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = ShellApp::new(run_command);
    app.run().await?;
    Ok(())
}

