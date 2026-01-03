use std::process::{Command, Stdio};
use wishlist::config::Config;

fn main() {
    let config = Config::load().expect("Failed to load configuration");

    let mut cmd = Command::new("sea-orm-cli");
    cmd.arg("generate")
        .arg("entity")
        .arg("--output-dir")
        .arg("src/entities")
        .env("DATABASE_URL", &config.database.url)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    let status = cmd.status().expect("Failed to execute sea-orm-cli");

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
}
