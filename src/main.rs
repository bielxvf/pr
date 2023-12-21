extern crate home;
use std::fs;
use std::path::PathBuf;
use std::env;

const PROGRAM_NAME: &str = "pr";

fn get_config_path() -> PathBuf {
    match home::home_dir() {
        Some(mut home_dir) => {
            home_dir.push(".config");
            home_dir.push(PROGRAM_NAME);
            home_dir
        }
        None => {
            eprintln!("Why the fuck do you not have a HOME env variable");
            std::process::exit(1);
        }
    }
}

fn check_config_dir(path: PathBuf) {
    if !path.exists() {
        eprintln!("INFO: Creating missing config path: {}", path.as_path().display().to_string());
        match fs::create_dir_all(path.clone()) {
            Ok(()) => eprintln!("INFO: Config directory created"),
            Err(e) => eprintln!("Error: Could not create config directory: {e:?}"),
        }
        assert!(path.exists(), "Unreachable: config path somehow disappeared?");
    }

}

fn cmd_help() {
    println!("Usage: {} [command] [...]", PROGRAM_NAME);
    println!("    Commands:");
    println!("        help\tPrint this message");
    println!("        list\tList password directory");
    println!();

    std::process::exit(0);
}

fn cmd_list() {
    let path = get_config_path();
    println!("Contents of {}:", path.as_path().display().to_string());
    // TODO
    std::process::exit(0);
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        cmd_help();
    }

    let ref subcommand = args.remove(1);

    let config_path = get_config_path();

    check_config_dir(config_path);


    match subcommand.as_str() {
        "help" => {
            if args.len() > 1 {
                println!("Too many args");
                std::process::exit(1);
            }
            cmd_help();
        }
        "list" => {
            if args.len() > 1 {
                println!("Too many args");
                std::process::exit(1);
            }
            cmd_list();
        }
        _ => {
            println!("Unknown subcommand '{}'", subcommand);
            std::process::exit(1);
        }
    }

}
