use std::net::{IpAddr, SocketAddr};
use std::time::{Duration, Instant};
use std::mem::MaybeUninit;
use std::io::{self, Write};
use colored::Colorize;
use std::process::Command;

use anyhow::{Context, Result};
use clap::Parser;
use pnet::packet::{
    icmp::{echo_request::MutableEchoRequestPacket, IcmpPacket, IcmpTypes},
    ipv4::Ipv4Packet,
    Packet,
};
use socket2::{Domain, Protocol, Socket, Type};
use pnet::packet::icmp::echo_request::EchoRequestPacket;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short = 'm', long, default_value_t = 30)]
    max_hops: u8,

    #[clap(short = 'q', long, default_value_t = 3)]
    attempts: u8,

    #[clap(short = 'w', long, default_value_t = 1)]
    timeout: u64,
}

pub(crate) fn route_tracing() -> Result<()> {
    let args = Args::parse();

    let target = get_target_from_user()?;
    println!("Tracing route to {}...", target);

    let recv_socket = Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::ICMPV4))?;
    recv_socket.set_read_timeout(Some(Duration::from_secs(args.timeout)))?;

    for ttl in 1..=args.max_hops {
        print!("{:2}  ", ttl);
        let mut got_response = false;

        for attempt in 0..args.attempts {
            let sequence = ttl * args.attempts + attempt;

            let send_socket = Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::ICMPV4))?;
            send_socket.set_ttl(ttl as u32)?;

            let request = build_icmp_echo_request(0, sequence as u16);
            let dest = SocketAddr::new(target, 0).into();

            let start = Instant::now();
            if send_socket.send_to(&request, &dest).is_err() {
                print!(" {} ", "Err".red());
                continue;
            }

            let mut response_received = false;
            while start.elapsed() < Duration::from_secs(args.timeout) {
                let mut buffer = [MaybeUninit::uninit(); 1024];
                match recv_socket.recv_from(&mut buffer) {
                    Ok((size, _)) => {
                        let buffer = unsafe { std::mem::transmute::<_, &[u8]>(&buffer[..size]) };
                        if let Some((src, seq)) = parse_icmp_response(buffer) {
                            if seq == sequence as u16 {
                                let elapsed = start.elapsed();
                                print!(" {} {:.2}ms ", src, elapsed.as_millis());
                                response_received = true;
                                got_response = true;
                                break;
                            }
                        }
                    }
                    Err(_) => break,
                }
            }

            if !response_received {
                print!(" {} ", "Ok".green());
            }
        }

        println!();
        if got_response && target == resolve_target(&target.to_string())? {
            break;
        }
    }
    ask_for_restart();
    Ok(())
}

fn get_target_from_user() -> Result<IpAddr> {
    loop {
        console_clear();
        println!("\n ██▀███   ▒█████   █    ██ ▄▄▄█████▓ ██▀███   ▄▄▄       ▄████▄  \n▓██ ▒ ██▒▒██▒  ██▒ ██  ▓██▒▓  ██▒ ▓▒▓██ ▒ ██▒▒████▄    ▒██▀ ▀█  \n▓██ ░▄█ ▒▒██░  ██▒▓██  ▒██░▒ ▓██░ ▒░▓██ ░▄█ ▒▒██  ▀█▄  ▒▓█    ▄ \n▒██▀▀█▄  ▒██   ██░▓▓█  ░██░░ ▓██▓ ░ ▒██▀▀█▄  ░██▄▄▄▄██ ▒▓▓▄ ▄██▒\n░██▓ ▒██▒░ ████▓▒░▒▒█████▓   ▒██▒ ░ ░██▓ ▒██▒ ▓█   ▓██▒▒ ▓███▀ ░\n░ ▒▓ ░▒▓░░ ▒░▒░▒░ ░▒▓▒ ▒ ▒   ▒ ░░   ░ ▒▓ ░▒▓░ ▒▒   ▓▒█░░ ░▒ ▒  ░\n  ░▒ ░ ▒░  ░ ▒ ▒░ ░░▒░ ░ ░     ░      ░▒ ░ ▒░  ▒   ▒▒ ░  ░  ▒   \n  ░░   ░ ░ ░ ░ ▒   ░░░ ░ ░   ░        ░░   ░   ░   ▒   ░        \n   ░         ░ ░     ░                 ░           ░  ░░ ░      \n                                                       ░        ");
        println!("Enter the IP address {} to trace: ", "(DON'T DOMAIN NAME)".yellow());
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        match resolve_target(input) {
            Ok(ip) => {console_clear(); return Ok(ip)},
            Err(_) => print!("{} {}", "Error: invalid address.".red(), "Try again.".red()),
        }
    }
}

fn resolve_target(target: &str) -> Result<IpAddr> {
    let addr = format!("{}:0", target)
        .parse::<SocketAddr>()
        .context("Invalid target address")?
        .ip();
    Ok(addr)
}

fn build_icmp_echo_request(identifier: u16, sequence: u16) -> Vec<u8> {
    let mut buffer = vec![0u8; 8];
    let mut packet = MutableEchoRequestPacket::new(&mut buffer).unwrap();
    packet.set_icmp_type(IcmpTypes::EchoRequest);
    packet.set_icmp_code(pnet::packet::icmp::IcmpCode(0));
    packet.set_identifier(identifier);
    packet.set_sequence_number(sequence);
    let checksum = pnet::packet::util::checksum(packet.packet(), 1);
    packet.set_checksum(checksum);
    buffer
}

fn parse_icmp_response(packet: &[u8]) -> Option<(IpAddr, u16)> {
    let ipv4_packet = Ipv4Packet::new(packet)?;
    let icmp_packet = IcmpPacket::new(ipv4_packet.payload())?;

    match icmp_packet.get_icmp_type().0 {
        0 => {
            let echo_reply = EchoRequestPacket::new(icmp_packet.packet())?;
            Some((ipv4_packet.get_source().into(), echo_reply.get_sequence_number()))
        }
        11 => {
            let payload = icmp_packet.payload();
            if payload.len() < 28 {
                return None;
            }
            let sequence = u16::from_be_bytes([payload[26], payload[27]]);
            Some((ipv4_packet.get_source().into(), sequence))
        }
        _ => None,
    }
}

fn ask_for_restart() {
    loop {
        println!("\nChoose an option:",);
        println!("1)Return to main menu",);
        println!("2)Check another IP",);
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        let _ = io::stdin().read_line(&mut choice);
        let choice = choice.trim();

        match choice {
            "1" => {console_clear(); return},
            "2" => {
                console_clear();
                let _ = route_tracing();
            },
            _ => {
                console_clear();
                println!("{}", "Invalid choice! Please enter 1 or 2".red());
                continue;
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