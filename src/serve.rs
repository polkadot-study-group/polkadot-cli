use std::process::{Command, ExitStatus};
use std::path::Path;


pub fn make_executable(script_path: &Path) -> Result<ExitStatus, String> {
    Command::new("chmod")
        .arg("+x")
        .arg(script_path)
        .status()
        .map_err(|e| format!("Failed to make script executable: {}", e))
}

pub fn run_script(script_path: &Path, args: &[&str]) -> Result<ExitStatus, String> {
    Command::new("bash")
        .arg(script_path)
        .args(args)
        .status()
        .map_err(|e| format!("Failed to run script: {}", e))
}

pub fn run(args: &[&str]) {
    println!("Running omni-node...");

    let script_path = Path::new("./script/omni-node.sh");

    match make_executable(script_path) {
        Ok(status) if status.success() => {
            println!("Script is executable.");
        }
        _ => {
            eprintln!("Failed to make script executable");
            return;
        }
    }

    println!("Running script: {:?}", script_path);

    match run_script(script_path, args) {
        Ok(status) if status.success() => {
            println!("Omni-node is now running.");
        }
        _ => {
            eprintln!("Failed to run script at {:?}", script_path);
        }
    }
}

// pub fn run(args: &[&str]) {
//     println!("Running omni-node...");

//     // Define the path to the script
//     let script_path = Path::new("./script/omni-node.sh");

//     // Make the script executable
//     let chmod_status = Command::new("chmod")
//         .arg("+x")
//         .arg(script_path)
//         .status()
//         .expect("Failed to make script executable");

//     if !chmod_status.success() {
//         eprintln!("Failed to make script executable");
//         return;
//     }
//     println!("Running script: {:?}", script_path);

//     let status = Command::new("bash")
//         .arg(script_path)
//         .args(args)
//         .status()
//         .expect("Failed to run script");

//     if !status.success() {
//         eprintln!("Failed to run script at {:?}", script_path);
//         return;
//     }

//     println!("Omni-node is now running.");
// }
