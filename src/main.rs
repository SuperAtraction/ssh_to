mod machine;

use std::io;
use std::fs;
use serde::Deserialize;
use std::path::PathBuf;
use machine::Machine;

#[derive(Deserialize)]
struct Config {
    machine: Vec<Machine>,
}

fn get_config_path() -> PathBuf {
    let config_dir = dirs::config_dir().expect("Unable to find config directory").join("ssh-manager");
    fs::create_dir_all(&config_dir).expect("Unable to create config directory");
    config_dir.join("machines.toml")
}

fn main() {
    let config_path = get_config_path();
    println!("Config path: {}", config_path.display());

    if !config_path.exists() {
        fs::write(&config_path, "").expect("Unable to create configuration file");
    }

    let content = fs::read_to_string(&config_path).expect("Unable to read machines.toml");
    if content.trim().is_empty() {
        eprintln!("Configuration file is empty. Please add machines to machines.toml.");
        return;
    }

    let config: Config = match toml::from_str(&content) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Configuration file error: {e}");
            return;
        }
    };
    let machines = config.machine;

    println!("Your machines:");
    for (i, machine) in machines.iter().enumerate() {
        println!(
            "{}: {} as {} @ {}",
            i + 1,
            machine.name,
            machine.user,
            machine.ip
        );
    }

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .unwrap();

    let position: usize = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Please enter a valid number.");
            return;
        }
    };

    if position == 0 {
        println!("Invalid number.");
        return;
    }

    if let Some(machine) = machines.get(position - 1) {
        println!("\nConnecting to {} ({})...", machine.name, machine.ip);

        match machine.connect() {
            Ok(code) => println!("SSH finished with code {}", code),
            Err(e) => println!("{}", e),
        }
    } else {
        println!("Machine not found.");
    }
}
