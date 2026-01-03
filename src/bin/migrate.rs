use std::process::{Command, Stdio};
use wishlist::config::Config;

fn main() {
    let config = Config::load().expect("Failed to load configuration");

    let mut cmd = Command::new("sea-orm-cli");
    cmd.arg("migrate")
        .arg("--migration-dir")
        .arg("migration")
        .env("DATABASE_URL", &config.database.url)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    // Get the subcommand from command line args (up, down, status, etc.)
    // Skip the first arg (binary name) and collect the rest
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        // Default to "up" if no subcommand provided
        cmd.arg("up");
    } else {
        // Pass all remaining args as the subcommand and its arguments
        for arg in args {
            cmd.arg(&arg);
        }
    }

    let status = cmd.status().expect("Failed to execute sea-orm-cli");

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}
