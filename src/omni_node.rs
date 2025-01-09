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
        }
    }


    let nodes_dir = Path::new("./nodes");
    if !nodes_dir.exists() {
        println!("Creating nodes directory...");
        if let Err(e) = fs::create_dir_all(nodes_dir) {
            eprintln!("Failed to create nodes directory: {}", e);
            // return;
        }
    }

    // Move the binary to the desired location
    let source_path = repo_path.join("target/release/polkadot-omni-node");
    let destination_path = Path::new("./nodes/polkadot-omni-node");
    println!("Moving binary to: {:?}", destination_path);
    if let Err(e) = fs::rename(&source_path, &destination_path) {
        eprintln!("Failed to move the binary: {}", e);
        // return;
    }

    println!("Omni-node installation complete.");

    gen_omni_node_bin(repo_path);

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

pub fn gen_omni_node_bin(repo_path: &Path){
    let wasm_source_path = repo_path.join("target/release/wbuild/asset-hub-westend-runtime/asset_hub_westend_runtime.compact.compressed.wasm");
    // Check if the wasm file exists
    // Check if the wasm file exists
    // if wasm_source_path.exists() {
    
        // Build the project if the wasm file does not exist
        // let status = Command::new("cargo")
        //     .args(&["build", "--release", "--package", "asset-hub-westend-runtime"])
        //     .current_dir(&repo_path)
        //     .status()
        //     .expect("Failed to build project");
        let status = Command::new("cargo")
            .args(&["install", "--git", "https://github.com/paritytech/polkadot-sdk", "--force", "staging-chain-spec-builder"])
            .current_dir(&repo_path)
            .status()
            .expect("Failed to install project");

        if !status.success() {
            eprintln!("Failed to build project");
            return;
        }

        // Check again if the wasm file was created
        if !wasm_source_path.exists() {
            eprintln!("Wasm file not found after build: {:?}", wasm_source_path);
            return;
        }
    // }

    // Run the chain-spec-builder command
    println!("Wasm file already exists: {:?}", wasm_source_path);
    // Ensure the file has read permissions before proceeding
    let chmod_status = Command::new("chmod")
        .args(&["+r", wasm_source_path.to_str().unwrap()])
        .status()
        .expect("Failed to run chmod");

    if !chmod_status.success() {
        eprintln!("Failed to add read permissions to the WASM file");
        // Return or handle error
        return;
    }

    let chain_spec_status = Command::new("chain-spec-builder")
        .args(&[
            "create",
            "-t", "development",
            "--relay-chain", "paseo",
            "--para-id", "1000",
            "--runtime", wasm_source_path.to_str().unwrap(),
            "named-preset", "development"
        ])
        .current_dir(&repo_path)
        .status()
        .expect("Failed to run chain-spec-builder");

    if !chain_spec_status.success() {
        eprintln!("Failed to run chain-spec-builder");
        // return;
    }

    // Define the path to the chain_spec.json file
    let chain_spec_source_path = repo_path.join("chain_spec.json");
    let chain_spec_destination_path = Path::new("./chain-specs/chain_spec.json");

    // Move the chain_spec.json file to the chain-specs directory
    if chain_spec_source_path.exists() {
        println!("Moving chain_spec.json to: {:?}", chain_spec_destination_path);
        if let Err(e) = fs::rename(&chain_spec_source_path, &chain_spec_destination_path) {
            eprintln!("Failed to move chain_spec.json: {}", e);
            return;
        }
    } else {
        eprintln!("chain_spec.json not found: {:?}", chain_spec_source_path);
        return;
    }

    println!("Omni-node binary and chain spec generated successfully.");

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