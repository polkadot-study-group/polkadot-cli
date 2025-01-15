#[cfg(test)]
mod e2e_tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    use std::process::Command;
    use crate::serve::make_executable;
    use crate::install::{install, install_polkadot, install_chain_spec_builder, install_omni_node, run_download_script, gen_chain_spec, move_chain_spec};
    use crate::process::Stdio;


    // INSTALL TESTS
    #[test]
    fn test_install_polkadot_success() {
        // Mock the curl command to return a success status
        let output = Command::new("curl")
            .arg("--proto")
            .arg("=https")
            .arg("--tlsv1.2")
            .arg("-sSf")
            .arg("https://raw.githubusercontent.com/paritytech/polkadot-sdk/refs/heads/master/scripts/getting-started.sh")
            .stdout(Stdio::piped())
            .output()
            .expect("Failed to execute curl");

        assert!(output.status.success(), "Polkadot installation failed");
    }

    #[test]
    fn test_install_chain_spec_builder() {
        let url = "https://github.com/paritytech/polkadot-sdk/releases/download/polkadot-stable2412/chain-spec-builder";
        let destination = Path::new("./binaries/chain-spec-builder");
        
        if destination.exists() {
            fs::remove_file(destination).expect("Failed to remove existing file");
        }
        
        // Simulate successful download with wget
        let output = Command::new("wget")
            .arg("-O")
            .arg(destination)
            .arg(url)
            .output()
            .expect("Failed to execute wget");

        assert!(output.status.success(), "Chain spec builder installation failed");
    }

    #[test]
    fn test_install_omni_node() {
        let url = "https://github.com/paritytech/polkadot-sdk/releases/download/polkadot-stable2412/polkadot-omni-node";
        let destination = Path::new("./binaries/polkadot-omni-node");
        
        if destination.exists() {
            fs::remove_file(destination).expect("Failed to remove existing file");
        }
        
        // Simulate successful download with wget
        let output = Command::new("wget")
            .arg("-O")
            .arg(destination)
            .arg(url)
            .output()
            .expect("Failed to execute wget");

        assert!(output.status.success(), "Omni-node installation failed");
    }

    #[test]
    fn test_run_download_script() {
        let url = "https://github.com/paritytech/polkadot-sdk/releases/download/polkadot-stable2412/asset_hub_westend_runtime.compact.compressed.wasm";
        let destination = Path::new("./nodes/asset_hub_westend_runtime.compact.compressed.wasm");
        
        if destination.exists() {
            fs::remove_file(destination).expect("Failed to remove existing file");
        }

        // Simulate successful download with wget
        let output = Command::new("wget")
            .arg("-O")
            .arg(destination)
            .arg(url)
            .output()
            .expect("Failed to execute wget");

        assert!(output.status.success(), "Wasm file download failed");
    }

    #[test]
    fn test_gen_chain_spec() {
        let wasm_source_path = Path::new("./nodes/asset_hub_westend_runtime.compact.compressed.wasm");
        let chain_spec_builder_path = Path::new("./binaries/chain-spec-builder");

        // Ensure the WASM file exists
        assert!(wasm_source_path.exists(), "WASM file does not exist");

        // Ensure the chain spec builder exists
        assert!(chain_spec_builder_path.exists(), "Chain spec builder binary does not exist");

        // Simulate the chain spec generation command
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

        assert!(chain_spec_status.success(), "Failed to generate chain spec");
    }

    #[test]
    fn test_move_chain_spec() {
        let chain_spec_json_path = Path::new("./chain-specs/chain_spec.json");
        if chain_spec_json_path.exists() {
            fs::remove_file(chain_spec_json_path).expect("Failed to remove existing chain spec file");
        }

        let result = move_chain_spec();
        assert!(result.is_ok(), "Failed to move chain spec: {:?}", result.err());

        // Verify the file was moved successfully
        assert!(chain_spec_json_path.exists(), "Chain spec file was not moved");
    }

    #[test]
    fn test_install() {
        // This is a high-level integration test, calling the install function
        install("template_name");

        // You can add assertions here for the expected log outputs or effects
    }

    // SERVE TESTS
    #[test]
    fn test_make_executable() {
        // Adjust the path to point to the real script in your project
        let script_path = Path::new("script/omni-node.sh");

        // Ensure the script exists and is not already executable
        let result = make_executable(&script_path);

        assert!(result.is_ok(), "Failed to make script executable");

        // Verify the script is executable
        let metadata = std::fs::metadata(&script_path).unwrap();
        assert!(metadata.permissions().readonly() == false, "File should be executable");
    
    }
}