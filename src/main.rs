use clap::{Parser, Subcommand};
use std::{
    io::{stdin, BufRead},
    net::SocketAddr,
};

mod vrchat_box;

#[derive(Subcommand, Debug, Clone, Copy)]
enum Commands {
    /// Clears the chatbox
    Clear,
    /// Print version and other program info
    Version,
}

#[derive(Debug, Parser)]
#[command(version, disable_version_flag = true)]
struct Args {
    /// Print version and other program info
    #[arg(long = "version", short = 'V')]
    show_version: bool,
    /// Always send full stdin as message
    #[arg(long, short = 'a')]
    dont_replace: bool,
    /// Enable chatbox notification sound
    #[arg(long, short)]
    sfx: bool,
    /// Open ingame keyboard with any input
    #[arg(long, short)]
    keyboard: bool,
    /// Show typing indicator for entire runtime
    #[arg(long, short)]
    typing_indicator: bool,
    /// Address to send messages to
    #[arg(long, short = 'S', default_value_t = vrchat_box::VRCHAT_OSC_ADDR)]
    server_address: SocketAddr,
    /// Port to open on to send messages from
    #[arg(long, short = 'C', default_value_t = 0)]
    client_port: u16,
    /// Send once and exit
    #[arg(num_args = 1..)]
    prompt: Option<Vec<String>>,
    #[command(subcommand)]
    cmd: Option<Commands>,
}

fn main() {
    let cli = Args::parse();
    run(cli);
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
        cmd,
        show_version,
    }: Args,
) {
    let vrcclient = vrchat_box::ClientBuilder::new()
        .with_client_port(client_port)
        .with_server_ip(server_address.ip())
        .with_server_port(server_address.port())
        .build()
        .unwrap();

    if show_version {
        version();
        return;
    }

    if let Some(subcmd) = cmd {
        match subcmd {
            Commands::Clear => {
                _ = vrcclient.send_message("", true, false);
            }
            Commands::Version => {
                version();
            }
        }
        return;
    }

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

    _ = vrcclient.typing_indicator(false);
}

fn version() {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("{}", env!("CARGO_PKG_DESCRIPTION"));
    println!("hosted at {}", env!("CARGO_PKG_REPOSITORY"));
}
