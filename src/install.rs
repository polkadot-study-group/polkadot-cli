use std::process::Command;
use std::fs;
use std::process::Stdio;
use std::path::{Path, PathBuf};

pub fn install(_template: &str){
    let mut results = Vec::new();

    results.push((install_polkadot(), "$ Polkadot installation"));
    results.push((install_chain_spec_builder(), "$ Chain spec builder installation"));
    results.push((install_omni_node(), "$ Omni-node installation"));
    results.push((run_download_script(), "$ Wasm file download script"));
    results.push((gen_chain_spec(), "$ Chain spec script"));

    // Print the results after all operations
    println!(" ");
    println!("===========================================================================");
    println!(" ");
    for (result, message) in results {
        match result {
            Ok(_) => println!("{} success ✓", message),
            Err(_) => println!("{} failed ✗", message),
        }
    }
    println!(" ");
    println!("===========================================================================");
    println!(" ");
}

pub fn install_polkadot() -> Result<(), String>{
    println!("Installing Polkadot via curl");

    let url = "https://raw.githubusercontent.com/paritytech/polkadot-sdk/refs/heads/master/scripts/getting-started.sh"; 
    
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
        return Err(format!("Failed to run Polkadot-sdk"));
    }

    println!("Polkadot-sdk is now installed.");
    Ok(()) 
}

pub fn install_chain_spec_builder() -> Result<(), String>{
    println!("Installing chain-spec-builder");
    
    let status = Command::new("cargo")
        .args(&["install", "--git", "https://github.com/paritytech/polkadot-sdk", "--force", "staging-chain-spec-builder"])
        .status()
        .expect("Failed to install chain-spec-builder");

    if !status.success() {
        eprintln!("Failed to build chain-spec-builder");
        return Err(format!("Failed to build chain-spec-builder"));
    }

    Ok(()) 
}

pub fn install_omni_node() -> Result<(), String>{
    println!("Installing omni-node");

    let repo_path = Path::new("./templates");

    let status = Command::new("cargo")
        .args(&["install", "--git", "https://github.com/paritytech/polkadot-sdk", "--force", "polkadot-omni-node"])
        .current_dir(&repo_path)
        .status()
        .expect("Failed to install chain-spec-builder");

    if !status.success() {
        return Err(format!("Failed to build omni-node"));
    }

    println!("Omni-node installation complete.");

    Ok(())
}

pub fn run_download_script() -> Result<(), String>{
    let script_path = Path::new("./script/download-runtime-wasm.sh");

    // Make the script executable
    let chmod_status = Command::new("chmod")
        .arg("+x")
        .arg(script_path)
        .status()
        .expect("Failed to make script executable");

    if !chmod_status.success() {
        eprintln!("Failed to make script executable");
        return Err(format!("Failed to make script executable"));
    }
    println!("Running script: {:?}", script_path);

    // Run the command with the specified arguments
    let status = Command::new("bash")
        .arg(script_path)
        .status()
        .expect("Failed to run script");

    if !status.success() {
        return Err(format!("Failed to run script at {:?}", script_path));
    }
    Ok(())
}


pub fn gen_chain_spec() -> Result<(), String>{
    let wasm_source_path =  Path::new("./nodes/asset_hub_westend_runtime.compact.compressed.wasm");

    // Check if the WASM file exists
    if !wasm_source_path.exists() {
        eprintln!("WASM file not found: {:?}", wasm_source_path);
        return Err(format!("WASM file not found: {:?}", wasm_source_path));
    }

    let chmod_status = Command::new("chmod")
        .args(&["+r", wasm_source_path.to_str().unwrap()])
        .status()
        .expect("Failed to run chmod");

    if !chmod_status.success() {
        eprintln!("Failed to add read permissions to the WASM file");
        return Err(format!("Failed to add read permissions to the WASM file"));
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
    }
    let _ = move_chain_spec();
    Ok(()) 
}

pub fn move_chain_spec() -> Result<(), String>{
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
            return Err(format!("chain_spec.json not found in the specified directories."));
        }
    };
    
    let chain_spec_destination_path = Path::new("./chain-specs/chain_spec.json");

    // Create the chain-specs directory if it does not exist
    if let Err(e) = fs::create_dir_all(chain_spec_destination_path.parent().unwrap()) {
        return Err(format!("Failed to create chain-specs directory: {}", e));
    }
    
    // Move the chain_spec.json file to the chain-specs directory
    if let Err(e) = fs::rename(&chain_spec_source_path, &chain_spec_destination_path) {
        return Err(format!("Failed to move chain_spec.json: {}", e));
    }
    Ok(())
}