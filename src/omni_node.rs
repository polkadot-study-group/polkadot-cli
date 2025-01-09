use std::process::Command;
use std::path::Path;
use std::fs;
use std::io::{self};

pub fn add(_template: &str) {
    println!("Installing omni-node");

    let repo_path = Path::new("./templates/polkadot-sdk");
    let binary_path= Path::new("./nodes/polkadot-omni-node");

    if binary_path.exists() {
        println!("polkadot-omni-node binary already exists. Skipping installation.");
    }else{
        // Check if the repository exists
        if repo_path.exists() {
            println!("polkadot-sdk directory already exists. Skipping cloning.");
        }else{
            println!("No existing polkadot-sdk directory found.");
            println!("Would you like to clone the repository? (Y/n)");
            // Read user input
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read input");
            // Trim and check the input
            let input = input.trim();
            if input.is_empty() || input.trim().eq_ignore_ascii_case("Y") {
                
                // Installation logic: clone the repository 
                let status = Command::new("git")
                    .args(&["clone", "https://github.com/paritytech/polkadot-sdk.git", repo_path.to_str().unwrap()])
                    .status()
                    .expect("Failed to clone repository");

                if !status.success() {
                    eprintln!("Failed to clone repository");
                    return;
                }
            } else {
                println!("Exiting...");
                return;
            }
        }
    }

    // Build the omni-node
    let omni_node_path = repo_path.join("cumulus/polkadot-omni-node");
    let status = Command::new("cargo")
        .args(&["build", "--release"])
        .current_dir(&omni_node_path)
        .status()
        .expect("Failed to build project");

    if !status.success() {
        eprintln!("Failed to build project");
        return;
    }


    let nodes_dir = Path::new("./nodes");
    if !nodes_dir.exists() {
        println!("Creating nodes directory...");
        if let Err(e) = fs::create_dir_all(nodes_dir) {
            eprintln!("Failed to create nodes directory: {}", e);
            return;
        }
    }

    // Move the binary to the desired location
    let source_path = repo_path.join("target/release/polkadot-omni-node");
    let destination_path = Path::new("./nodes/polkadot-omni-node");
    println!("Moving binary to: {:?}", destination_path);
    if let Err(e) = fs::rename(&source_path, &destination_path) {
        eprintln!("Failed to move the binary: {}", e);
        return;
    }

    println!("Omni-node installation complete.");
}

pub fn run(args: &[&str]) {
    println!("Running omni-node...");


    // Define the path to the script
    let script_path = Path::new("./script/omni-node.sh");

    // Make the script executable
    let chmod_status = Command::new("chmod")
        .arg("+x")
        .arg(script_path)
        .status()
        .expect("Failed to make script executable");

    if !chmod_status.success() {
        eprintln!("Failed to make script executable");
        return;
    }
    println!("Running script: {:?}", script_path);

    // Run the command with the specified arguments
    let status = Command::new(script_path)
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
        let repo_path = Path::new("./templates/polkadot-sdk");
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