#[cfg(test)]
mod e2e_tests {
    use std::process::Command;

    #[test]
    fn test_cli_run() {
        let output = Command::new("cargo")
            .args(&["run", "--", "run", "zombienet", "--help"])
            .output()
            .expect("Failed to execute command");

        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Usage"));
    }
}