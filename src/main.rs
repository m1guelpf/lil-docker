#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use anyhow::Result;
use hub::Hub;
use std::{
    env,
    process::{exit, Command, Stdio},
};

mod hub;
mod image;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<_> = env::args().collect();

    if args[1] != "run" {
        eprintln!("Usage: {} run <image> <command> [args...]", args[0]);
        exit(1);
    }

    let image = &args[2];
    let command = &args[3];
    let command_args = &args[4..];

    let image = Hub::pull(image).await;

    let exit_status = image.run(
        Command::new(command)
            .args(command_args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit()),
    );

    exit(exit_status.code().unwrap_or(1));
}
