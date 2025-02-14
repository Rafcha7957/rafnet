use colored::Colorize;
use reqwest::Client;
use std::time::Instant;
use tokio::task;
use std::process::Command;

pub async fn load_test(url: &str, requests: usize) {
    let client = Client::new();
    let start = Instant::now();

    let tasks: Vec<_> = (0..requests)
        .map(|i| {
            let client = client.clone();
            let url = url.to_string();
            task::spawn(async move {
                let response = client.get(&url).send().await.unwrap();
                println!(
                    "Request {}: Status = {}, Time = {:?}",
                    i + 1,
                    response.status(),
                    start.elapsed()
                );
            })
        })
        .collect();

    for task in tasks {
        task.await.unwrap();
    }

    println!("Completed {} requests in {:?}", requests, start.elapsed());
}

#[tokio::main]
pub async fn loadtest() {
    loop {
        console_clear();
        println!("\n ██▓     ▒█████   ▄▄▄      ▓█████▄ ▄▄▄█████▓▓█████   ██████ ▄▄▄█████▓\n▓██▒    ▒██▒  ██▒▒████▄    ▒██▀ ██▌▓  ██▒ ▓▒▓█   ▀ ▒██    ▒ ▓  ██▒ ▓▒\n▒██░    ▒██░  ██▒▒██  ▀█▄  ░██   █▌▒ ▓██░ ▒░▒███   ░ ▓██▄   ▒ ▓██░ ▒░\n▒██░    ▒██   ██░░██▄▄▄▄██ ░▓█▄   ▌░ ▓██▓ ░ ▒▓█  ▄   ▒   ██▒░ ▓██▓ ░ \n░██████▒░ ████▓▒░ ▓█   ▓██▒░▒████▓   ▒██▒ ░ ░▒████▒▒██████▒▒  ▒██▒ ░ \n░ ▒░▓  ░░ ▒░▒░▒░  ▒▒   ▓▒█░ ▒▒▓  ▒   ▒ ░░   ░░ ▒░ ░▒ ▒▓▒ ▒ ░  ▒ ░░   \n░ ░ ▒  ░  ░ ▒ ▒░   ▒   ▒▒ ░ ░ ▒  ▒     ░     ░ ░  ░░ ░▒  ░ ░    ░    \n  ░ ░   ░ ░ ░ ▒    ░   ▒    ░ ░  ░   ░         ░   ░  ░  ░    ░      \n  ░ ░   ░ ░ ░ ▒    ░   ▒    ░ ░  ░   ░         ░   ░  ░  ░    ░      \n    ░  ░    ░ ░        ░  ░   ░                ░  ░      ░           \n                            ░                                        ");
    
        println!("Enter target URL:");
        let mut url = String::new();
        std::io::stdin()
            .read_line(&mut url)
            .expect("Failed to read URL");
        let url = url.trim();
    
        let url = if !url.starts_with("http://") && !url.starts_with("https://") {
            format!("https://{}", url)
        } else {
            url.to_string()
        };
    
        println!("Enter number of requests:");
        let mut requests = String::new();
        std::io::stdin()
            .read_line(&mut requests)
            .expect(&"Failed to read number of requests".red());
    
        let requests: usize = requests
            .trim()
            .parse()
            .expect(&"Please enter a valid positive number".red());
    
        load_test(&url, requests).await;

        println!("\nChoose an option:",);
        println!("1)Return to main menu",);
        println!("2)Test another url",);
        let mut input = String::new();

        std::io::stdin().read_line(&mut input).unwrap();
        let number: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                console_clear();
                print!("{}\n", "Error: An invalid value has been entered. Please enter the number specified in the list.".red());
                break;
            }
        };
        match number {
            1 => {console_clear(); return},
            2 => {console_clear(); continue},
            _ => {
                console_clear();
                print!("{}\n", "Error: An invalid value has been entered. Please enter the number specified in the list.".red());
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