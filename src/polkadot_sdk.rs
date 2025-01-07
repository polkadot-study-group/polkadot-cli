use std::process::Command;
use std::path::Path;
use std::io::{self};

pub fn install(template: &str) {
    println!("Installing omni-node with template: {}...", template);

    // Define the path where the repository will be cloned
    let repo_path = Path::new("./dependencies/polkadot-sdk");

    // Remove the existing directory if it exists
    if repo_path.exists() {
        println!("polkadot-sdk directory already exists. Skipping cloning.");
        // fs::remove_dir_all(repo_path).expect("Failed to remove existing omni-node directory");
    }else{
        println!("No existing omni-node directory found.");

        // Read user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        // Trim and check the input
        if input.trim().eq_ignore_ascii_case("Y") {
            
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



    // Build the omni-node
    // let omni_node_path = repo_path.join("cumulus/polkadot-omni-node");
    let status = Command::new("bash")
        .arg("./dependencies/polkadot-sdk/scripts/getting-started.sh")
        .status()
        .expect("Failed to run project");

    if !status.success() {
        eprintln!("Failed to run project");
        return;
    }

    println!("Omni-node is now running.");
}
