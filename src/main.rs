use std::process;
use clap::{App, Command};

mod serve;
mod template;
mod install;
mod os_check;
mod test;


fn main() {
    let os_info = os_check::get_os_info();
    println!("{}", os_info);
    
    let matches = App::new("polkadot-cli")
        .version("0.1.0")
        .author("Author Name <author@example.com>")
        .about("CLI tool for Polkadot")
        .usage("dot [SUBCOMMAND] --template [ minimal | solochain | parachain ]")
        .subcommand(
            Command::new("install")
                .about("Install operation does the following:
- Install polkadot dependencies
- Get polkadot-omni-node binary
- Get chain-spec-builder binary
- Get runtime wasm file
- Generate chain spec file
                ")
                .arg(
                    clap::Arg::new("template")
                        .help("The template to use for installation")
                        .long("template")
                        .takes_value(true), // These options are listed here,
                )
                .arg(
                    clap::Arg::new("chain")
                        .help("Use --template to install and run a template")
                        .required(false)
                        .index(1), // Adding the possible values,
                )
                .arg(
                    clap::Arg::new("args")
                        .help("Template available arguments
                        minimal 
                        parachain 
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
                    clap::Arg::new("chain")
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



fn handle_install(matches: &clap::ArgMatches) {
    if let Some(template_name) = matches.value_of("template") {
        let args: Vec<&str> = matches.values_of("args").unwrap_or_default().collect();
        template::run_template(&args, template_name);
    } else if let Some(chain) = matches.value_of("chain") {
        match chain {
            "minimal" | "parachain" | "solochain" => {
                let args: Vec<&str> = matches.values_of("args").unwrap_or_default().collect();
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