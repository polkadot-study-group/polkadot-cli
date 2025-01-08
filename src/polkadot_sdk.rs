use std::process::Command;
use std::path::Path;
use std::io::{self};
use std::process::Stdio;


pub fn install(_template: &str) {
    println!("Installing Polkadot via curl");

    let url = "https://raw.githubusercontent.com/paritytech/polkadot-sdk/refs/heads/master/scripts/getting-started.sh"; 
    // Build the omni-node
    
    // Run the curl command and pipe its output to bash
    let curl = Command::new("curl")
        .arg("--proto")
        .arg("=https")
        .arg("--tlsv1.2")
        .arg("-sSf")
        .arg(url)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start curl");
    
    let status = Command::new("bash")
        .stdin(curl.stdout.unwrap())
        .status()
        .expect("Failed to run bash");

    if !status.success() {
        eprintln!("Failed to run project");
        return;
    }

    println!("Polkadot-sdk is now installed.");
}

pub fn clone(template: &str) {
    println!("Installing polkadot-sdk with template: {}...", template);

    // Define the path where the repository will be cloned
    let repo_path = Path::new("./templates/polkadot-sdk");

    // Remove the existing directory if it exists
    if repo_path.exists() {
        println!("polkadot-sdk directory already exists. Skipping cloning.");
        // fs::remove_dir_all(repo_path).expect("Failed to remove existing omni-node directory");
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



pub fn run(_args: &[&str]) {
    println!("Running polkadot-sdk...");

    // Define the path where the repository was cloned
    // Build the omni-node
    let status = Command::new("bash")
        .arg("./templates/polkadot-sdk/scripts/getting-started.sh")
        .status()
        .expect("Failed to run project");

    if !status.success() {
        eprintln!("Failed to run project");
        return;
    }

    println!("Omni-node is now running.");
}