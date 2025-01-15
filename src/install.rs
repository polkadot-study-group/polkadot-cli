use std::process::Command;
use std::fs;
use std::process::Stdio;
use std::path::{Path, PathBuf};
use std::error::Error;
use crate::os_check;

pub fn install(_template: &str){
    let mut results: Vec<(Result<(), Box<dyn Error>>, &str)> = Vec::new();

    results.push((install_polkadot(), "$ Polkadot installation"));
    results.push((install_chain_spec_builder(), "$ Chain spec builder installation"));
    results.push((install_omni_node(), "$ Omni-node installation"));
    results.push((run_download_script(), "$ Wasm file download script"));
    results.push((gen_chain_spec(), "$ Chain spec script"));

    println!(" ");
    println!("===========================================================================");
    println!(" ");
    for (result, message) in results {
        match result {
            Ok(_) => println!("{} success ✓", message),
            Err(_e) => println!("{} failed ✗", message),
        }
    }
    println!(" ");
    println!("===========================================================================");
    println!(" ");
}

pub fn install_polkadot() -> Result<(), Box<dyn Error>>{
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
        return Err(format!("Failed to run Polkadot-sdk").into());
    }

    println!("Polkadot-sdk is now installed.");
    Ok(()) 
}

pub fn install_chain_spec_builder() -> Result<(), Box<dyn Error>> {
    println!("Installing chain-spec-builder");

    // Determine the operating system and set the appropriate URL
    let os_info = os_check::get_os_info();
    let url;
    if os_info.as_str() == "linux" {
        url = "https://github.com/paritytech/polkadot-sdk/releases/download/polkadot-stable2412/chain-spec-builder";
    } else if os_info.as_str() == "macos" {
        url = "https://binary.xode.net/chain-spec-builder";
    } else {
        return Err(format!("Unsupported OS: {}", os_info).into());
    }

    // Destination file path
    let destination = Path::new("./binaries/chain-spec-builder");
    if destination.exists() {
        println!("Chain-spec-builder binary is available");
        return Ok(());
    }

    // Check if the 'binaries' directory exists, if not, create it
    let binaries_dir = Path::new("./binaries");
    if !binaries_dir.exists() {
        println!("'binaries' directory does not exist. Creating it...");
        if let Err(e) = fs::create_dir_all(binaries_dir) {
            return Err(format!("Failed to create 'binaries' directory: {}", e).into());
        }
    }

    println!("Downloading...");
    let output = Command::new("wget")
        .arg("-O")
        .arg(destination)
        .arg(url)
        .output()
        .map_err(|e| format!("Failed to execute wget: {}", e))?;

    // Check if the download was successful
    if output.status.success() {
        println!("Download successful: {:?}", destination);

        let destination_str = destination.to_str().expect("Failed to convert path to str");

        let _chmod_status = Command::new("chmod")
            .args(&["755", destination_str])
            .status()
            .expect("Failed to run chmod");

        return Ok(());
    } else {
        return Err(format!(
            "Download failed with exit code: {:?}",
            output.status.code()
        )
        .into());
    }
}



pub fn install_omni_node() -> Result<(), Box<dyn Error>> {
    println!("Installing polkadot-omni-node");

    // Determine the operating system and set the appropriate URL
    let os_info = os_check::get_os_info();
    let url;
    if os_info.as_str() == "linux" {
        url = "https://github.com/paritytech/polkadot-sdk/releases/download/polkadot-stable2412/polkadot-omni-node";
    } else if os_info.as_str() == "macos" {
        url = "https://binary.xode.net/polkadot-omni-node";
    } else {
        return Err(format!("Unsupported OS: {}", os_info).into());
}

    // Destination file path
    let destination = Path::new("./binaries/polkadot-omni-node");
    if destination.exists() {
        println!("Omni-node binary is available");
        return Ok(());
    }

    // Check if the 'binaries' directory exists, if not, create it
    let binaries_dir = Path::new("./binaries");
    if !binaries_dir.exists() {
        println!("'binaries' directory does not exist. Creating it...");
        if let Err(e) = fs::create_dir_all(binaries_dir) {
            return Err(format!("Failed to create 'binaries' directory: {}", e).into());
        }
    }

    println!("Downloading...");
    let output = Command::new("wget")
        .arg("-O")
        .arg(destination)
        .arg(url)
        .output()
        .map_err(|e| format!("Failed to execute wget: {}", e))?;

    // Check if the download was successful
    if output.status.success() {
        println!("Download successful: {:?}", destination);

        let destination_str = destination.to_str().expect("Failed to convert path to str");

        let _chmod_status = Command::new("chmod")
            .args(&["755", destination_str])
            .status()
            .expect("Failed to run chmod");

        return Ok(());
    } else {
        return Err(format!(
            "Download failed with exit code: {:?}",
            output.status.code()
        )
        .into());
    }
}

pub fn run_download_script() -> Result<(), Box<dyn Error>>{
    let url = "https://github.com/paritytech/polkadot-sdk/releases/download/polkadot-stable2412/asset_hub_westend_runtime.compact.compressed.wasm";
    
    // Destination file path
    let destination = Path::new("./nodes/asset_hub_westend_runtime.compact.compressed.wasm");
    if destination.exists() {
        println!("Wasm file is available");
        return Ok(())
    }
    
    // Check if the 'binaries' directory exists, if not, create it
    let nodes_dir = Path::new("./nodes");
    if !nodes_dir.exists() {
        println!("'nodes' directory does not exist. Creating it...");
        if let Err(e) = fs::create_dir_all(nodes_dir) {
            return Err(format!("Failed to create 'nodes' directory: {}", e).into());
        }
    }
    
    println!("Downloading...");
    let output = Command::new("wget")
        .arg("-O")
        .arg(destination)
        .arg(url)
        .output()
        .map_err(|e| format!("Failed to execute wget: {}", e))?;
    
    // Check if the download was successful
    if output.status.success() {
        println!("Download successful: {:?}", destination);
        return Ok(())
    } else {
        return Err(format!(
            "Download failed with exit code: {:?}",
            output.status.code()
        ).into());
    }
}


pub fn gen_chain_spec() -> Result<(), Box<dyn Error>>{
    let wasm_source_path =  Path::new("./nodes/asset_hub_westend_runtime.compact.compressed.wasm");
    let chain_spec_builder_path = Path::new("./binaries/chain-spec-builder");

    // Check if the WASM file exists
    if !wasm_source_path.exists() {
        eprintln!("WASM file not found: {:?}", wasm_source_path);
        return Err(format!("WASM file not found: {:?}", wasm_source_path).into());
    }

    let chmod_status = Command::new("chmod")
        .args(&["+r", wasm_source_path.to_str().unwrap()])
        .status()
        .expect("Failed to run chmod");

    if !chmod_status.success() {
        eprintln!("Failed to add read permissions to the WASM file");
        return Err(format!("Failed to add read permissions to the WASM file").into());
    }

    // Add execute permissions to the chain-spec-builder binary
    let chmod_chain_spec_status = Command::new("chmod")
        .args(&["+x", chain_spec_builder_path.to_str().unwrap()])
        .status()
        .expect("Failed to run chmod on chain-spec-builder");

    if !chmod_chain_spec_status.success() {
        eprintln!("Failed to add execute permissions to the chain-spec-builder");
        return Err(format!("Failed to add execute permissions to the chain-spec-builder").into());
    }

    // let chain_spec_status = Command::new("chain-spec-builder")
    let chain_spec_status = Command::new("./binaries/chain-spec-builder")
        .args(&[
            "create",
            "-t", "development",
            "--relay-chain", "westend2",
            "--para-id", "1000",
            "--runtime", wasm_source_path.to_str().unwrap(),
            "named-preset", "development"
        ])
        .status()
        .expect("Failed to run chain-spec-builder");

    if !chain_spec_status.success() {
        return Err(format!("Failed to run chain-spec-builder").into());
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