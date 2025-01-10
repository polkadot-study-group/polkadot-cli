use std::process::Command;
use std::path::{Path, PathBuf};
use std::fs;
use std::io::{self};

// use crate::polkadot_sdk;

pub fn add(_template: &str) {
    println!("Installing omni-node");

    let repo_path = Path::new("./templates");
    let binary_path= Path::new("./nodes/polkadot-omni-node");

    let status = Command::new("cargo")
        .args(&["install", "--git", "https://github.com/paritytech/polkadot-sdk", "--force", "polkadot-omni-node"])
        .current_dir(&repo_path)
        .status()
        .expect("Failed to install chain-spec-builder");

    if !status.success() {
        eprintln!("Failed to build project");
        return;
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

    // gen_omni_node_bin(repo_path);

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
    let status = Command::new("bash")
        .arg(script_path)
        .args(args)
        .status()
        .expect("Failed to run script");

    if !status.success() {
        eprintln!("Failed to run script at {:?}", script_path);
        return;
    }

    println!("Omni-node is now running.");
}

pub fn install_chain_spec_builder(_template: &str){
    let repo_path = Path::new("./templates/polkadot-sdk");
    println!("Installing chain-spec-builder");
    
    let status = Command::new("cargo")
        .args(&["install", "--git", "https://github.com/paritytech/polkadot-sdk", "--force", "staging-chain-spec-builder"])
        .current_dir(&repo_path)
        .status()
        .expect("Failed to install chain-spec-builder");

    if !status.success() {
        eprintln!("Failed to build project");
        return;
    }

}

pub fn gen_chain_spec(_template: &str){
    let repo_path = Path::new("./templates/polkadot-sdk");

    let wasm_source_path = repo_path.join("target/release/wbuild/asset-hub-westend-runtime/asset_hub_westend_runtime.compact.compressed.wasm");

    // Check if the wasm file exists
    if !wasm_source_path.exists() {
        eprintln!("Wasm file not found after build: {:?}", wasm_source_path);
        return;
    }

    
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
        .status()
        .expect("Failed to run chain-spec-builder");

    if !chain_spec_status.success() {
        eprintln!("Failed to run chain-spec-builder");
        // return;
    }
    move_chain_spec();
}

pub fn move_chain_spec(){
    // let repo_path = Path::new("./templates/polkadot-sdk");

    // Define the directory to search for the chain_spec.json file
    let search_directories = vec!["./", "../"];
    let mut chain_spec_source_path: Option<PathBuf> = None;

    // Locate the chain_spec.json file
    for dir in &search_directories {
        let potential_path = Path::new(dir).join("chain_spec.json");
        if potential_path.exists() {
            chain_spec_source_path = Some(potential_path);
            break;
        }
    }

    // Check if the file was found
    let chain_spec_source_path = match chain_spec_source_path {
        Some(path) => path,
        None => {
            eprintln!("chain_spec.json not found in the specified directories.");
            return;
        }
    };
    
    println!("ChainSpec file location {:?}", chain_spec_source_path);
    let chain_spec_destination_path = Path::new("./chain-specs/chain_spec.json");
    
    // Move the chain_spec.json file to the chain-specs directory
    if let Err(e) = fs::rename(&chain_spec_source_path, &chain_spec_destination_path) {
        eprintln!("Failed to move chain_spec.json: {}", e);
        return;
    }

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