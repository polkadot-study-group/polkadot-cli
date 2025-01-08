use std::process;
use clap::{App, Arg, SubCommand};

mod zombienet;
mod omni_node;
mod polkadot_sdk;
mod template;


fn main() {

    let matches = App::new("polkadot-cli")
        .version("1.0")
        .author("Author Name <author@example.com>")
        .about("CLI tool for Polkadot")
        .subcommand(
            SubCommand::with_name("add")
                .about("Installs a specified chain")
                .arg(
                    Arg::with_name("CHAIN")
                        .help("The name of the chain to install")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("template")
                        .help("The template to use for installation")
                        .long("template")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("install")
                .about("Installs the polkadot-sdk")
                .arg(
                    Arg::with_name("CHAIN")
                        .help("The name of the component to install")
                        .required(false)
                        .index(1),
                )
        )
        .subcommand(
            SubCommand::with_name("run")
                .about("Runs a specified chain")
                .arg(
                    Arg::with_name("CHAIN")
                        .help("The name of the chain to run")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("ARGS")
                        .help("Arguments to pass to the chain")
                        .multiple(true)
                        .last(true),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("add") {
        if let Some(chain) = matches.value_of("CHAIN") {
            match chain {
                "zombienet" => {
                    let template = matches.value_of("template").unwrap_or("default");
                    zombienet::add(template);
                    process::exit(0);
                }
                "omni-node" => {
                    let template = matches.value_of("template").unwrap_or("default");
                    omni_node::add(template);
                    process::exit(0);
                }
                _ => {
                    println!("Unknown chain: {}", chain);
                    process::exit(1);
                }
            }
        }
    } else if let Some(_matches) = matches.subcommand_matches("install") {
        polkadot_sdk::clone("default");
        polkadot_sdk::install("curl");
        println!("Polkadot-sdk installed.");
        process::exit(0);
    } else if let Some(matches) = matches.subcommand_matches("run") {
        if let Some(chain) = matches.value_of("CHAIN") {
            println!("Running chain: {}", chain);
            match chain {
                "minimal" | "parachain" | "solochain" => {
                    let args: Vec<&str> = matches.values_of("ARGS").unwrap_or_default().collect();
                    template::run_template(&args, chain);
                    process::exit(0);
                }
                "zombienet" => {
                    let args: Vec<&str> = matches.values_of("ARGS").unwrap_or_default().collect();
                    zombienet::run(&args);
                    process::exit(0);
                }
                "omni-node" => {
                    let args: Vec<&str> = matches.values_of("ARGS").unwrap_or_default().collect();
                    omni_node::run(&args);
                    process::exit(0);
                }
                "polkadot-sdk" => {
                    let args: Vec<&str> = matches.values_of("ARGS").unwrap_or_default().collect();
                    polkadot_sdk::run(&args);
                    process::exit(0);
                }
                _ => {
                    println!("Unknown chain: {}", chain);
                    process::exit(1);
                }
            }
        }
    } else {
        println!("No valid subcommand provided.");
        process::exit(1);
    }
}