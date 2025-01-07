use std::process::Command;
use std::path::Path;
use std::fs;

pub fn add(template: &str) {
    println!("Installing omni-node with template: {}...", template);

    // Define the path where the repository will be cloned
    let repo_path = Path::new("./dependencies/polkadot-sdk");

    // Remove the existing directory if it exists
    if repo_path.exists() {
        println!("Removing existing omni-node directory...");
        fs::remove_dir_all(repo_path).expect("Failed to remove existing omni-node directory");
    }

    // Installation logic: clone the repository 
    let status = Command::new("git")
        .args(&["clone", "https://github.com/paritytech/polkadot-sdk.git", repo_path.to_str().unwrap()])
        .status()
        .expect("Failed to clone repository");

    if !status.success() {
        eprintln!("Failed to clone repository");
        return;
    }

    // Build the omni-node
    // let omni_node_path = repo_path.join("cumulus/polkadot-omni-node");
    let status = Command::new("cargo")
        .args(&["build", "--release"])
        .current_dir(&repo_path)
        .status()
        .expect("Failed to build project");

    if !status.success() {
        eprintln!("Failed to build project");
        return;
    }

    println!("Omni-node installation complete.");
}


pub fn run(args: &[&str]) {
    println!("Running omni-node...");

    // Define the path where the repository was cloned
    let repo_path = Path::new("./dependencies/polkadot-sdk/cumulus/polkadot-omni-node");
    println!("args: {:?}", args);
    // Example run logic: run the built node with arguments
    let status = Command::new(repo_path.join("target/release/polkadot-omni-node"))
        .args(args)
        .status()
        .expect("Failed to run project");

    if !status.success() {
        eprintln!("Failed to run project");
        return;
    }

    println!("Omni-node is now running.");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_install() {
        // Clean up before test
        let repo_path = Path::new("./dependencies/polkadot-sdk");
        if repo_path.exists() {
            fs::remove_dir_all(repo_path).expect("Failed to remove existing polkadot-sdk directory");
        }

        // Test install function
        add("default");

        // Verify installation
        assert!(repo_path.exists());
    }

    #[test]
    fn test_run() {
        // Ensure the install function has been run before testing run
        add("default");

        // Test run function
        run(&["-h"]);
    }
}