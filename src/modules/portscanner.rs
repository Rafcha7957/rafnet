use std::io;
use std::net::SocketAddr;
use std::process::Command;
use std::time::Duration;
use colored::Colorize;
use tokio::net::TcpStream as AsyncTcpStream;
use tokio::time::timeout;

async fn scan_port(addr: SocketAddr, timeout_ms: u64) -> bool {
    let result = timeout(Duration::from_millis(timeout_ms), AsyncTcpStream::connect(addr)).await;
    result.is_ok()
}

#[tokio::main]
pub async fn portsscanner() {
    console_clear();
    loop {
        println!(
            " ██▓███   ▒█████   ██▀███  ▄▄▄█████▓  ██████  ▄████▄   ▄▄▄       ███▄    █ \n▓██░  ██▒▒██▒  ██▒▓██ ▒ ██▒▓  ██▒ ▓▒▒██    ▒ ▒██▀ ▀█  ▒████▄     ██ ▀█   █ \n▓██░ ██▓▒▒██░  ██▒▓██ ░▄█ ▒▒ ▓██░ ▒░░ ▓██▄   ▒▓█    ▄ ▒██  ▀█▄  ▓██  ▀█ ██▒\n▒██▄█▓▒ ▒▒██   ██░▒██▀▀█▄  ░ ▓██▓ ░   ▒   ██▒▒▓▓▄ ▄██▒░██▄▄▄▄██ ▓██▒  ▐▌██▒\n▒██▒ ░  ░░ ████▓▒░░██▓ ▒██▒  ▒██▒ ░ ▒██████▒▒▒ ▓███▀ ░ ▓█   ▓██▒▒██░   ▓██░\n▒▓▒░ ░  ░░ ▒░▒░▒░ ░ ▒▓ ░▒▓░  ▒ ░░   ▒ ▒▓▒ ▒ ░░ ░▒ ▒  ░ ▒▒   ▓▒█░░ ▒░   ▒ ▒ \n░▒ ░       ░ ▒ ▒░   ░▒ ░ ▒░    ░    ░ ░▒  ░ ░  ░  ▒     ▒   ▒▒ ░░ ░░   ░ ▒░\n░░       ░ ░ ░ ▒    ░░   ░   ░      ░  ░  ░  ░          ░   ▒      ░   ░ ░ \n             ░ ░     ░                    ░  ░ ░            ░  ░         ░ \n                                             ░                             "
        );

        println!("Enter the port to be tested: ");

        let mut port_input = String::new();
        io::stdin()
            .read_line(&mut port_input)
            .expect("Error)");
        
        let port: u16 = match port_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                console_clear();
                println!("{}", "Error: wrong port entered".red());
                continue;
            }
        };

        let target = SocketAddr::from(([1, 1, 1, 1], port));
        let is_open = scan_port(target, 1000).await;
        println!("Port {} is open: {}", target.port(), is_open);

        println!("1)get out\n2)check more port");

        let mut inp = String::new();
        io::stdin().read_line(&mut inp).unwrap();

        let numm: u16 = match inp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                console_clear();
                println!("{}", "Error: wrong number entered".red());
                break;
            }
        }; 

        match numm {
            1 => {
                console_clear();
                return;
            },
            2 => {
                console_clear();
                continue;
            },
            _ => {
                console_clear();
                println!("{}", "Error: wrong number entered".red());
                break;
            }
        }

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