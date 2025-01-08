use std::process::Command;
use std::path::Path;

pub fn run_template(args: &[&str], template: &str) {
    println!("Running {}...{:?}", template, args);

    let destination = format!("./templates/{}-template", template);
    let destination_path = Path::new(&destination);

    if destination_path.exists() {
        println!("\n✅︎ {}-template directory already exists at {}. -> Entering.\n", template, destination);
    } else {
        println!("\n↓ Let's grab the {} template from github.\n", template);
        let status = Command::new("git")
            .args(&["clone", "--quiet", &format!("https://github.com/paritytech/polkadot-sdk-{}-template.git", template), &destination])
            .status()
            .expect("Failed to clone template");

        if !status.success() {
            eprintln!("Failed to clone template");
            return;
        }
    }

    println!("Entered directory: {}", destination);

    let repo_path = Path::new(&destination);
    println!("args: {:?}", args);

    let status = Command::new("cargo")
        .args(&["run", "--release", "--", "--dev"])
        .args(args)
        .current_dir(repo_path)
        .status()
        .expect("Failed to run project");

    if !status.success() {
        eprintln!("Failed to run project");
        return;
    }

    println!("{} is now running.", template);
}