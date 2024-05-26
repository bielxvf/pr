extern crate home;
use std::fs;
use std::path::PathBuf;

const PROGRAM_NAME: &str = "pr";
const PROGRAM_VERSION: &str = "0.1.0";

fn get_passwords_path() -> PathBuf {
    match home::home_dir() {
        Some(mut home_dir) => {
            home_dir.push(".config");
            home_dir.push(PROGRAM_NAME);
            home_dir.push("passwords"); // ~/.config/pr/passwords
            home_dir
        }
        None => {
            eprintln!("[ERROR] 'HOME' environment variable not set");
            std::process::exit(1);
        }
    }
}

fn check_config_dir(path: PathBuf) {
    if !path.exists() {
        eprintln!("[INFO] Creating missing passwords directory: {}", path.as_path().display().to_string());
        match fs::create_dir_all(path.clone()) {
            Ok(()) => eprintln!("[INFO] Passwords directory created"),
            Err(e) => eprintln!("[ERROR] Could not create config directory: {e:?}"),
        }
        assert!(path.exists(), "[UNREACHABLE] config path somehow disappeared");
    }
}

fn cmd_help() -> Result<(), ()> {
    println!("Usage: {} [command] [...]", PROGRAM_NAME);
    println!("Commands:");
    println!("\thelp Print this message");
    println!("\tlist List passwords");
    Ok(())
}

fn cmd_version() -> Result<(), ()> {
    println!("{} version: {}", PROGRAM_NAME, PROGRAM_VERSION);
    Ok(())
}

fn cmd_list() -> Result<(), std::io::Error> {
    let passwords_path = get_passwords_path();
    println!("Contents of {}:", passwords_path.as_path().display().to_string());
    check_config_dir(passwords_path.clone());
    for entry in fs::read_dir(passwords_path)? {
        let entry = entry?;
        println!("\t{:?}", entry.file_name());
    }
    Ok(())
}

fn cmd_new() -> Result<(), ()> { // TODO: Read password, master password, encrypt and save to file
}

fn main() {
    let subcmd = std::env::args().nth(1).expect("[ERROR] No subcommand given");

    match subcmd.as_str() {
        "help" => {
            cmd_help().expect("[ERROR] Unexpected error while executing subcommand 'help'")
        }
        "version" => {
            cmd_version().expect("[ERROR] Unexpected error while executing subcommand 'version'")
        }
        "list" => {
            cmd_list().expect("[ERROR] Unexpected error while executing subcommand 'list'")
        }
        _ => {
            eprintln!("[ERROR] Unknown subcommand '{}'", subcmd);
            std::process::exit(1);
        }
    }
}
