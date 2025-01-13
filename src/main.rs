use std::process;
use clap::{App, Arg, SubCommand, Command};
use std::env;

mod args;
mod serve;
mod template;
mod install;

use args::CliArgs;
use clap::Parser;

fn main() {
    let _args = CliArgs::parse();
    check_operating_system();
    
    let matches = App::new("polkadot-cli")
        .version("0.1.0")
        .author("Author Name <author@example.com>")
        .about("CLI tool for Polkadot")
        .subcommand(
            SubCommand::with_name("install")
                .about("Installs the polkadot-sdk")
                .arg(
                    Arg::new("template")
                        .help("The template to use for installation")
                        .long("template")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("CHAIN")
                        .help("Use --template to install and run a template")
                        .required(false)
                        .index(1),
                )
                .arg(
                    Arg::new("ARGS")
                        .help("Template available arguments \n
                        minimal \n
                        parachain \n
                        solochain
                        ")
                        .multiple(true)
                        .last(true),
                ),  
        )
        .subcommand(
            Command::new("serve")
                .about("Serve omni-node using westend asset hub runtime")
                .arg(
                    Arg::new("CHAIN")
                        .help("The name of the component to install")
                        .required(false)
                        .index(1),
                )
        )
    .get_matches();


    match matches.subcommand() {
        Some(("install", sub_matches)) => handle_install(sub_matches),
        Some(("serve", sub_matches)) => handle_serve(sub_matches),
        _ => {
            eprintln!("No valid subcommand provided. Use --help for more information.");
            process::exit(1);
        }
    }
}

fn check_operating_system() {
    let os = env::consts::OS;
    match os {
        "macos" => println!("The operating system is macOS."),
        "linux" => println!("The operating system is Linux."),
        "windows" => {
            if is_wsl() {
                println!("The operating system is Windows running under WSL2.");
            } else {
                println!("The operating system is Windows.");
            }
        },
        _ => println!("Unknown operating system: {}", os),
    }
}
// Function to check if the OS is WSL
fn is_wsl() -> bool {
    // Check for the presence of the WSL environment variable
    std::path::Path::new("/proc/version").exists() && 
    std::fs::read_to_string("/proc/version").unwrap_or_default().contains("Microsoft")
}

fn handle_install(matches: &clap::ArgMatches) {
    if let Some(template_name) = matches.value_of("template") {
        let args: Vec<&str> = matches.values_of("ARGS").unwrap_or_default().collect();
        template::run_template(&args, template_name);
    } else if let Some(chain) = matches.value_of("CHAIN") {
        match chain {
            "minimal" | "parachain" | "solochain" => {
                let args: Vec<&str> = matches.values_of("ARGS").unwrap_or_default().collect();
                template::run_template(&args, chain);
            }
            _ => {
                eprintln!("Invalid chain provided: {}", chain);
                process::exit(1);
            }
        }
    } else {
        install::install("default");
        println!("Environment is ready.");
    }
    process::exit(0);
}

fn handle_serve(matches: &clap::ArgMatches) {
    let args: Vec<&str> = matches.values_of("ARGS").unwrap_or_default().collect();
    serve::run(&args);
    process::exit(0);
}