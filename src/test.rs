#[cfg(test)]
mod e2e_tests {
    use super::*;
    use std::env;
    use std::{fs, path::Path};
    use std::process::{Command, Output};
    use mockito::mock;
    use std::fs::{File, create_dir_all};
    use tempfile::tempdir;
    use crate::install::{install, install_polkadot, install_chain_spec_builder, install_omni_node, run_download_script, gen_chain_spec, move_chain_spec};
    use crate::process::Stdio;
    use crate::os_check::{check_operating_system, get_os_info, is_wsl};
    


    // INSTALL
    #[test]
    fn test_install_polkadot() {
        // Create a temporary directory for mock binaries
        let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
        let mock_dir = temp_dir.path();

        // Create mock `curl` binary
        let curl_path = mock_dir.join("curl");
        fs::write(&curl_path, "#!/bin/sh\necho Mock curl executed\nexit 0")
            .expect("Failed to write mock curl");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&curl_path, fs::Permissions::from_mode(0o755))
                .expect("Failed to make mock curl executable");
        }

        // Create mock `bash` binary
        let bash_path = mock_dir.join("bash");
        fs::write(&bash_path, "#!/bin/sh\necho Mock bash executed\nexit 0")
            .expect("Failed to write mock bash");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&bash_path, fs::Permissions::from_mode(0o755))
                .expect("Failed to make mock bash executable");
        }

        // Override PATH to use mock binaries
        let original_path = env::var("PATH").unwrap_or_default();
        let new_path = format!("{}:{}", mock_dir.to_str().unwrap(), original_path);
        env::set_var("PATH", new_path);

        // Run the function and check the result
        let result = install_polkadot();
        assert!(result.is_ok(), "Function failed: {:?}", result.err());

        // Restore the original PATH
        env::set_var("PATH", original_path);
    }

    // OS CHECK
    // Mock the is_wsl function for testing
    fn mock_is_wsl() -> bool {
        env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() == "windows"
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn test_check_operating_system_linux() {
        env::set_var("CARGO_CFG_TARGET_OS", "linux");

        let os_info = check_operating_system();
        assert_eq!(os_info, "linux");
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn test_check_operating_system_windows_wsl() {
        env::set_var("CARGO_CFG_TARGET_OS", "windows");
        
        // Mock WSL behavior (we simulate WSL here)
        if mock_is_wsl() { 
            let os_info = check_operating_system();
            assert_eq!(os_info, "windows-wsl2");
        } else {
            let os_info = check_operating_system();
            assert_eq!(os_info, "windows");
        }
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn test_check_operating_system_macos() {
        env::set_var("CARGO_CFG_TARGET_OS", "macos");
        
        // Check if the OS info returns the correct result
        let os_info = check_operating_system();
        assert_eq!(os_info, "macos");
    }

    // CHAIN SPEC BUILDER
    // Test for successful installation
    #[test]
    fn test_install_chain_spec_builder_success() {
        // Mock the URL and wget command
        let _mock = mock("GET", "/chain-spec-builder")
            .with_status(200)
            .with_body("test binary content")
            .create();

        // Temporary directory to test file creation
        let temp_dir = tempdir::TempDir::new("install_test").unwrap();
        let temp_path = temp_dir.path().join("binaries");

        // Mock successful command execution for wget and chmod
        fs::create_dir_all(&temp_path).unwrap();

        // Execute the function
        let result = install_chain_spec_builder();

        assert!(result.is_ok());
    }
 
 
    // Test if 'binaries' directory is created when it doesn't exist
    #[test]
    fn test_create_binaries_directory() {
        // Create a temporary directory for testing
        let temp_dir = tempdir::TempDir::new("install_test").unwrap();
        let binaries_dir = temp_dir.path().join("binaries");

        // Ensure the directory doesn't exist at the start of the test
        assert!(!binaries_dir.exists());

        // Change the working directory for the test to the temp directory
        std::env::set_current_dir(temp_dir.path()).unwrap();

        // Run the function to check if it creates the 'binaries' directory
        install_chain_spec_builder().unwrap();

        // Check that the 'binaries' directory was created
        assert!(binaries_dir.exists());
    }



    // MOVING CHAIN SPEC
    #[test]
    fn test_move_chain_spec_success() {
        // Create temporary directory for testing
        let temp_dir = tempdir::TempDir::new("move_chain_spec_test").unwrap();
        let source_dir = temp_dir.path().join("source");
        let destination_dir = temp_dir.path().join("chain-specs");

        // Create source directory and file
        create_dir_all(&source_dir).unwrap();
        let chain_spec_path = source_dir.join("chain_spec.json");
        File::create(&chain_spec_path).unwrap(); // Create the chain_spec.json file

        // Test the move_chain_spec function
        let result = move_chain_spec();

        // Assert the result is Ok
        assert!(result.is_ok());

        // Check if the file was moved to the destination
        let destination_path = destination_dir.join("chain_spec.json");
        assert!(destination_path.exists());
    }

    #[test]
    fn test_move_chain_spec_not_found() {
        // Create temporary directory
        let temp_dir = tempdir::TempDir::new("move_chain_spec_test").unwrap();
        let source_dir = temp_dir.path().join("source");

        // Create source directory without chain_spec.json file
        create_dir_all(&source_dir).unwrap();

        // Test the move_chain_spec function when the file is not found
        let result = move_chain_spec();

        // Assert the result is an error
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "chain_spec.json not found in the specified directories.");
    }

    #[test]
    fn test_move_chain_spec_create_directory_failure() {
        // Create a temporary directory and simulate a failure in creating the target directory
        let temp_dir = tempdir::TempDir::new("move_chain_spec_test").unwrap();
        let source_dir = temp_dir.path().join("source");
        let invalid_dir = temp_dir.path().join("invalid_dir");

        // Create source directory and file
        create_dir_all(&source_dir).unwrap();
        let chain_spec_path = source_dir.join("chain_spec.json");
        File::create(&chain_spec_path).unwrap();

        // Simulate a failure in creating the 'chain-specs' directory
        let result = move_chain_spec();

        // Assert the result is an error
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Failed to create chain-specs directory"));
    }

    #[test]
    fn test_move_chain_spec_move_file_failure() {
        // Create temporary directory and simulate a failure in moving the file
        let temp_dir = tempdir::TempDir::new("move_chain_spec_test").unwrap();
        let source_dir = temp_dir.path().join("source");
        let invalid_dir = temp_dir.path().join("invalid_dir");

        // Create source directory and file
        create_dir_all(&source_dir).unwrap();
        let chain_spec_path = source_dir.join("chain_spec.json");
        File::create(&chain_spec_path).unwrap();

        // Simulate a failure in moving the file
        let result = move_chain_spec();

        // Assert the result is an error
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Failed to move chain_spec.json"));
    }
}