use std::process::{Command};

pub fn run(args: &[&str]) {
    println!("Running omni-node...");

    let command = Command::new("./binaries/polkadot-omni-node")
        .args(args) 
        .status();

    match command {
        Ok(status) if status.success() => {
            println!("Omni-node is now running.");
        }
        Ok(status) => {
            eprintln!("Omni-node failed to start with exit status: {}", status);
        }
        Err(e) => {
            eprintln!("Failed to run omni-node: {}", e);
        }
    }
}