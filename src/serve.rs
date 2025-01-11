use std::process::Command;
use std::path::Path;


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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_run() {
        // Test run function
        run(&["-h"]);
    }
}