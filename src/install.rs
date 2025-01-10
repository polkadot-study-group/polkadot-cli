use std::process::Command;
use std::path::Path;
use std::io::{self};
use std::process::Stdio;

pub fn install(_template: &str){
    install_polkadot();
    install_chain_spec_builder();
    install_omni_node();
}

pub fn install_polkadot() {
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

pub fn install_chain_spec_builder(){
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

pub fn install_omni_node() {
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

    println!("Omni-node installation complete.");

}
