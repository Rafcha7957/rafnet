use std::{io, process::{exit, Command}};

use colored::Colorize;
use modules::portscanner::portsscanner;
use modules::route_tracing::route_tracing;
use modules::load_testing::loadtest;
use modules::traffic_analys::trafanalys;

mod modules;
fn main() {
    console_clear();

    loop {
        println!("██▀███   ▄▄▄        █████▒███▄    █ ▓█████▄▄▄█████▓\n▓██ ▒ ██▒▒████▄    ▓██   ▒ ██ ▀█   █ ▓█   ▀▓  ██▒ ▓▒\n▓██ ░▄█ ▒▒██  ▀█▄  ▒████ ░▓██  ▀█ ██▒▒███  ▒ ▓██░ ▒░\n▒██▀▀█▄  ░██▄▄▄▄██ ░▓█▒  ░▓██▒  ▐▌██▒▒▓█  ▄░ ▓██▓ ░ \n░██▓ ▒██▒ ▓█   ▓██▒░▒█░   ▒██░   ▓██░░▒████▒ ▒██▒ ░ \n░ ▒▓ ░▒▓░ ▒▒   ▓▒█░ ▒ ░   ░ ▒░   ▒ ▒ ░░ ▒░ ░ ▒ ░░   \n  ░▒ ░ ▒░  ▒   ▒▒ ░ ░     ░ ░░   ░ ▒░ ░ ░  ░   ░    \n  ░░   ░   ░   ▒    ░ ░      ░   ░ ░    ░    ░      \n   ░           ░  ░                ░    ░  ░        \n                                                    ");
        let mut input = String::new();
        
        println!("Please, select what you want:\n");
        println!("1)load testing\n2)port scanner\n3)route tracing\n4)traffic analys\n5)exit");
        io::stdin().read_line(&mut input).expect("Error, lol");

        let number: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                console_clear();
                println!("{}", "Error: An invalid value has been entered. Please enter the number specified in the list.".red());
                continue;
            }
        };

        match number {
            1 => loadtest(),
            2 => portsscanner(),
            3 => route_tracing().expect("oh"),
            4 => trafanalys(),
            5 => {
                console_clear();
                exit(0);
            },
            _ => {
                console_clear();
                println!("{}", "Error: An invalid value has been entered. Please enter the number specified in the list.".red());
                continue;
            }   
    };
    }
}

fn console_clear() {
    if cfg!(target_os = "windows") {
        let _ = Command::new("cmd")
            .args(&["/C", "cls"])
            .status();
    } else {
        let _ = Command::new("clear")
            .status();
    }
}