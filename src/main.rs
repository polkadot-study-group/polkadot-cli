use std::process;
use clap::{App, Arg, SubCommand};

mod serve;
mod template;
mod install;


fn main() {

    let matches = App::new("polkadot-cli")
        .version("1.0")
        .author("Author Name <author@example.com>")
        .about("CLI tool for Polkadot")
        .subcommand(
            // SubCommand::with_name("install")
            //     .about("Installs the polkadot-sdk")
            //     .arg(
            //         Arg::with_name("CHAIN")
            //             .help("The name of the component to install")
            //             .required(false)
            //             .index(1),
            //     )
            //     .arg(
            //         Arg::with_name("ARGS")
            //             .help("Arguments to pass to the chain")
            //             .multiple(true)
            //             .last(true),
            //     ),
            SubCommand::with_name("install")
                .about("Installs the polkadot-sdk")
                .arg(
                    Arg::with_name("template")
                        .help("The template to use for installation")
                        .long("template")
                        .takes_value(true), // Indicates that this argument takes a value
                )
                .arg(
                    Arg::with_name("CHAIN")
                        .help("The name of the component to install")
                        .required(false)
                        .index(1),
                )
                .arg(
                    Arg::with_name("ARGS")
                        .help("Arguments to pass to the chain")
                        .multiple(true)
                        .last(true),
                ),  
        )
        .subcommand(
            SubCommand::with_name("serve")
                .about("Installs the polkadot-sdk")
                .arg(
                    Arg::with_name("CHAIN")
                        .help("The name of the component to install")
                        .required(false)
                        .index(1),
                )
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("install") {
        if let Some(template_name) = matches.value_of("template") {
            // If a template is specified, run the corresponding template logic
            let args: Vec<&str> = matches.values_of("ARGS").unwrap_or_default().collect();
            template::run_template(&args, template_name);
            process::exit(0);
        }
        if let Some(chain) = matches.value_of("CHAIN") {
            println!("Running chain: {}", chain);
            match chain {
                "minimal" | "parachain" | "solochain" => {
                    let args: Vec<&str> = matches.values_of("ARGS").unwrap_or_default().collect();
                    template::run_template(&args, chain);
                    process::exit(0);
                }
                _ => {
                    println!("Invalid chain provided.");
                    process::exit(1);
                }
            }
        }else{
            install::install("default");
            println!("Environment is ready.");
            process::exit(0);
        }
    } else if let Some(matches) = matches.subcommand_matches("serve") {
        let args: Vec<&str> = matches.values_of("ARGS").unwrap_or_default().collect();
        serve::run(&args);
        process::exit(0);
    } else {
        println!("No valid subcommand provided.");
        process::exit(1);
    }
}