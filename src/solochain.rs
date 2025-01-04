use std::process::Command;
use std::path::Path;
use std::fs;

pub fn install(template: &str) {
    println!("Installing solochain with template: {}...", template);

    // Define the path where the repository will be cloned
    let repo_path = Path::new("./crates/solochain-template");

    // Remove the existing directory if it exists
    if repo_path.exists() {
        println!("Removing existing solochain-template directory...");
        fs::remove_dir_all(repo_path).expect("Failed to remove existing solochain-template directory");
    }

    // Installation logic: clone the repository 
    let status = Command::new("git")
        .args(&["clone", "https://github.com/paritytech/polkadot-sdk-solochain-template.git", repo_path.to_str().unwrap()])
        .status()
        .expect("Failed to clone repository");

    if !status.success() {
        eprintln!("Failed to clone repository");
        return;
    }

    // Build the solochain-template
    let status = Command::new("cargo")
        .args(&["build", "--release"])
        .current_dir(repo_path)
        .status()
        .expect("Failed to build project");

    if !status.success() {
        eprintln!("Failed to build project");
        return;
    }

    println!("Solochain installation complete.");
}

pub fn run(args: &[&str]) {
    println!("Running solochain...");

    // Define the path where the repository was cloned
    let repo_path = Path::new("./crates/solochain-template/");
    println!("args: {:?}", args);
    // Example run logic: run the built node with arguments
    let status = Command::new(repo_path.join("target/release/solochain-template-node"))
        .args(args)
        .status()
        .expect("Failed to run project");

    if !status.success() {
        eprintln!("Failed to run project");
        return;
    }

    println!("Solochain is now running.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_install() {
        // You can add tests for the install function here
        install("default");
    }

    #[test]
    fn test_run() {
        // You can add tests for the run function here
        run(&["-h"]);
    }
}