extern crate home;
use std::fs;
use std::path::PathBuf;
use std::io::{Write, BufRead};
use sha2::{Sha512, Digest};

const PROGRAM_NAME: &str = "pr";
const PROGRAM_VERSION: &str = "0.1.0";

fn main() {
    let subcmd = std::env::args().nth(1).expect("[ERROR] No subcommand given");

    match subcmd.as_str() {
        "help" => {
            cmd_help().expect("[ERROR] Unexpected error while executing subcommand 'help'")
        }
        "version" => {
            cmd_version().expect("[ERROR] Unexpected error while executing subcommand 'version'")
        }
        "set" => {
            cmd_set().expect("[ERROR] Unexpected error while executing subcommand 'set'")
        }
        "list" => {
            cmd_list().expect("[ERROR] Unexpected error while executing subcommand 'list'")
        }
        "new" => {
            cmd_new().expect("[ERROR] Unexpected error while executing subcommand 'new'")
        }
        _ => {
            panic!("[ERROR] Unknown subcommand '{}'", subcmd);
        }
    }
}

fn get_passwords_path() -> PathBuf {
    match home::home_dir() {
        Some(mut home_dir) => {
            home_dir.push(".config");
            home_dir.push(PROGRAM_NAME);
            home_dir.push("passwords"); // ~/.config/pr/passwords
            home_dir
        }
        None => {
            panic!("[ERROR] 'HOME' environment variable not set");
        }
    }
}

fn get_master_password_path() -> PathBuf {
    match home::home_dir() {
        Some(mut home_dir) => {
            home_dir.push(".config");
            home_dir.push(PROGRAM_NAME);
            home_dir.push("master");
            home_dir
        }
        None => {
            panic!("[ERROR] 'HOME' environment variable not set");
        }
    }
}

fn check_config_dir(path: PathBuf) {
    if !path.exists() {
        eprintln!("[INFO] Creating missing passwords directory: {}", path.as_path().display().to_string());
        match fs::create_dir_all(path.clone()) {
            Ok(()) => eprintln!("[INFO] Passwords directory created"),
            Err(e) => panic!("[ERROR] Could not create config directory: {e:?}"),
        }
        assert!(path.exists(), "[UNREACHABLE] config path somehow disappeared");
    }
}

fn cmd_help() -> Result<(), ()> {
    println!("Usage: {} [command] [...]", PROGRAM_NAME);
    println!("Commands:");
    println!("\thelp Print this message");
    println!("\tset Set master password");
    println!("\tlist List passwords");
    println!("\tnew Create new password");
    Ok(())
}

fn cmd_version() -> Result<(), ()> {
    println!("{} version: {}", PROGRAM_NAME, PROGRAM_VERSION);
    Ok(())
}

fn cmd_set() -> Result<(), std::io::Error> {
    let master_password_path = get_master_password_path();
    if master_password_path.exists() {
        panic!("[ERROR] A master password has already been set");
    }

    let line = std::io::stdin().lock().lines().next().unwrap().unwrap();
    let mut hasher = sha2::Sha512::new();
    hasher.update(line);
    let hash = hasher.finalize();
    fs::write(master_password_path, hash)?;

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

fn cmd_new() -> Result<(), std::io::Error> {
    let passwords_path = get_passwords_path();
    check_config_dir(passwords_path.clone());

    let master_password_path = get_master_password_path();
    if !master_password_path.exists() {
        panic!("[ERROR] Master password has not been set. Please run subcommand 'set'");
    }

    print!("Master password: ");
    std::io::stdout().flush().unwrap();
    let master_password = std::io::stdin().lock().lines().next().unwrap().unwrap();
    let mut hasher = Sha512::new();
    hasher.update(master_password);
    let hash = hasher.finalize();

    let master_hash = fs::read(master_password_path)?;

    match hash.to_vec() == master_hash {
        true => {
        }
        _ => {
            panic!("[ERROR] Master password does not match");
        }
    }

    let new_password = std::io::stdin().lock().lines().next().unwrap().unwrap();
    // TODO: Encrypt

    Ok(())
}
