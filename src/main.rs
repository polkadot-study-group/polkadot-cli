// use std::env;
// use std::fs;
use std::process;
use clap::{App, Arg, SubCommand};

mod solochain;
mod zombienet;
mod omni_node;
mod polkadot_sdk;


fn main() {

    // GETTING ARGUMENTS FROM COMMAND LINE AND READ FILE CONTENTS
    //     let args: Vec<String> = std::env::args().collect();
    //     // println!("{:?}", args);
    //     let config: Config = Config::new(&args).unwrap_or_else(|err| {
    //         println!("Problem parsing arguments: {}", err);
    //         process::exit(1);
    //     });

    //     println!("Searching for {}", config.query);
    //     println!("In file {}", config.filename);

    //     if let Err(e) = polkadot_cli::run(config) {
    //         println!("Application error: {}", e);
    //         process::exit(1);
    //     }

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
                        .help("The name of the chain to install")
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
                "solochain" => {
                    let template = matches.value_of("template").unwrap_or("default");
                    solochain::add(template);
                    process::exit(0);
                }
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
    } else if let Some(matches) = matches.subcommand_matches("install") {
        if let Some(chain) = matches.value_of("CHAIN") {
            match chain {
                "polkadot-sdk" => {
                    polkadot_sdk::install("default");
                    process::exit(0);
                }
                _ => {
                    println!("Unknown chain: {}", chain);
                    process::exit(1);
                }
            }
        } else {
            // Call the add functions for each chain
            solochain::add("default");
            zombienet::add("default");
            omni_node::add("default");
            println!("All chains installed.");
            process::exit(0);
        }
    } else if let Some(matches) = matches.subcommand_matches("run") {
        if let Some(chain) = matches.value_of("CHAIN") {
            match chain {
                "solochain" => {
                    let args: Vec<&str> = matches.values_of("ARGS").unwrap_or_default().collect();
                    solochain::run(&args);
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