use clap::Parser;
use std::{
    io::{stdin, BufRead},
    net::SocketAddr,
};

mod vrchat_box;

#[derive(Debug, Parser)]
#[command(version)]
struct Args {
    /// Show full stdin for message instead of replacing the current message
    #[arg(long, short = 'a')]
    dont_replace: bool,
    /// Enable playing chatbox notification sound for other users
    #[arg(long, short)]
    sfx: bool,
    /// Replace default behavior and open ingame keyboard with the input
    #[arg(long, short)]
    keyboard: bool,
    /// Show typing indicator for the lifetime of this program
    #[arg(long, short)]
    typing_indicator: bool,
    /// Manually set the OSC address to send messages to
    #[arg(long, short = 'S', default_value_t = vrchat_box::VRCHAT_OSC_ADDR)]
    server_address: SocketAddr,
    /// Manually set the client port to send messages from, default should be fine as the port is chosen by the os
    #[arg(long, short = 'C', default_value_t = 0)]
    client_port: u16,
    /// Optionally replace default behavior and send this instead of stdin
    #[arg(num_args = 1..)]
    prompt: Option<Vec<String>>,
}

fn main() {
    let v = Args::parse();
    run(v);
}

fn run(
    Args {
        dont_replace,
        client_port,
        sfx,
        prompt,
        server_address,
        keyboard,
        typing_indicator,
    }: Args,
) {
    let vrcclient = vrchat_box::ClientBuilder::new()
        .with_client_port(client_port)
        .with_server_ip(server_address.ip())
        .with_server_port(server_address.port())
        .build()
        .unwrap();

    if typing_indicator {
        _ = vrcclient.typing_indicator(true);
    }

    if let Some(prompt) = prompt {
        _ = vrcclient.send_message(prompt.join(" ").as_str(), !keyboard, sfx);
    } else {
        let mut stdin = stdin().lock();
        let mut running_buffer = String::new();
        loop {
            let buffer = stdin.fill_buf().unwrap();
            let length = buffer.len();
            if length == 0 {
                break;
            }
            let string_buffer = String::from_utf8(buffer.to_vec()).unwrap();
            if !dont_replace {
                running_buffer = string_buffer;
            } else {
                running_buffer.push_str(&string_buffer);
            }
            stdin.consume(length);
            _ = vrcclient.send_message(running_buffer.as_str(), !keyboard, sfx);
        }
    }

    if typing_indicator {
        _ = vrcclient.typing_indicator(false);
    }
}

// fn version() {
//     println!("{} - {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
//     println!("{}", env!("CARGO_PKG_DESCRIPTION"));
//     println!("hosted at {}", env!("CARGO_PKG_REPOSITORY"));
// }
