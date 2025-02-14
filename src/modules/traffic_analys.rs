use pcap::{Capture, Device};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::{
    io::{self, Write},
    process::Command,
};

use colored::Colorize;

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn console_clear() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "cls"]).status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}

pub fn trafanalys() {
    loop {
        console_clear();
        println!(
            "
▄▄▄█████▓ ██▀███   ▄▄▄        █████▒▄▄▄       ███▄    █  ▄▄▄       ██▓   ▓██   ██▓  ██████ 
▓  ██▒ ▓▒▓██ ▒ ██▒▒████▄    ▓██   ▒▒████▄     ██ ▀█   █ ▒████▄    ▓██▒    ▒██  ██▒▒██    ▒ 
▒ ▓██░ ▒░▓██ ░▄█ ▒▒██  ▀█▄  ▒████ ░▒██  ▀█▄  ▓██  ▀█ ██▒▒██  ▀█▄  ▒██░     ▒██ ██░░ ▓██▄   
░ ▓██▓ ░ ▒██▀▀█▄  ░██▄▄▄▄██ ░▓█▒  ░░██▄▄▄▄██ ▓██▒  ▐▌██▒░██▄▄▄▄██ ▒██░     ░ ▐██▓░  ▒   ██▒
  ▒██▒ ░ ░██▓ ▒██▒ ▓█   ▓██▒░▒█░    ▓█   ▓██▒▒██░   ▓██░ ▓█   ▓██▒░██████▒ ░ ██▒▓░▒██████▒▒
  ▒ ░░   ░ ▒▓ ░▒▓░ ▒▒   ▓▒█░ ▒ ░    ▒▒   ▓▒█░░ ▒░   ▒ ▒  ▒▒   ▓▒█░░ ▒░▓  ░  ██▒▒▒ ▒ ▒▓▒ ▒ ░
    ░      ░▒ ░ ▒░  ▒   ▒▒ ░ ░       ▒   ▒▒ ░░ ░░   ░ ▒░  ▒   ▒▒ ░░ ░ ▒  ░▓██ ░▒░ ░ ░▒  ░ ░
  ░        ░░   ░   ░   ▒    ░ ░     ░   ▒      ░   ░ ░   ░   ▒     ░ ░   ▒ ▒ ░░  ░  ░  ░  
            ░           ░  ░             ░  ░         ░       ░  ░    ░  ░░ ░           ░  
                                                                          ░ ░              
        "
        );

        let device_name = read_input("Enter network device name (e.g., 'eth0'): ");
        let filter = read_input("Enter BPF filter (e.g., 'tcp', 'port 80'): ");

        let running = Arc::new(AtomicBool::new(true));
        let running_handle = running.clone();

        let capture_thread = thread::spawn(move || {
            let device = match Device::list()
                .unwrap_or_default()
                .into_iter()
                .find(|d| d.name == device_name)
            {
                Some(d) => d,
                none => {
                    eprintln!(
                        "{}: Device '{}' not found",
                        "Error".red(),
                        device_name
                    );
                    return;
                }
            };

            let mut cap = match Capture::from_device(device)
                .and_then(|c| c.timeout(1000).open())
            {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("{}: {}", "Error".red(), e);
                    return;
                }
            };

            if let Err(e) = cap.filter(&filter) {
                eprintln!("{}: {}", "Filter error".red(), e);
                return;
            }

            println!("\n{}", "Starting packet capture.".green());
            
            while running_handle.load(Ordering::Relaxed) {
                match cap.next() {
                    Ok(packet) => println!("Packet: {:?}", packet),
                    Err(e) => {
                        if e.to_string().contains("timeout") {
                            continue;
                        }
                        eprintln!("{}: {}", "Error".red(), e);
                        break;
                    }
                }
            }
        });

        let _ = io::stdin().read_line(&mut String::new());
        running.store(false, Ordering::Relaxed);
        capture_thread.join().unwrap();

        let choice = read_input("\n1. Return to main menu\n2. Capture again\n> ");
        match choice.as_str() {
            "1" => break,
            "2" => continue,
            _ => {
                println!("{}", "Invalid choice, returning to main menu".yellow());
                break;
            }
        }
    }
}