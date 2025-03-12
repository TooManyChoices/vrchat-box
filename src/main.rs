use std::{
    env::args,
    fmt::Display,
    io::{stdin, BufRead},
    net::{AddrParseError, SocketAddr},
    num::ParseIntError,
};

mod vrchat_box;
use vrchat_box::VRCHAT_OSC_ADDR;

#[derive(Debug)]
struct ProgramSettings {
    append_buffer: bool,
    client_port: u16,
    make_sound: bool,
    prompt: Option<String>,
    server_address: SocketAddr,
    show_keyboard: bool,
    typing_indicator: bool,
    do_help: bool,
    do_version: bool,
}

impl Default for ProgramSettings {
    fn default() -> Self {
        ProgramSettings {
            append_buffer: false,
            client_port: 0,
            make_sound: false,
            prompt: None,
            server_address: VRCHAT_OSC_ADDR,
            show_keyboard: false,
            typing_indicator: false,
            do_help: false,
            do_version: false,
        }
    }
}

fn main() {
    match parse_args() {
        Err(e) => {
            println!("Error when parsing {}: {}", e.0, e.1);
        }
        Ok(v) => {
            if v.do_help {
                help();
            } else if v.do_version {
                version();
            } else {
                run(v);
            }
        }
    }
}

fn run(
    ProgramSettings {
        append_buffer,
        client_port,
        make_sound,
        prompt,
        server_address,
        show_keyboard,
        typing_indicator,
        do_help: _,
        do_version: _,
    }: ProgramSettings,
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
        _ = vrcclient.send_message(prompt.as_str(), !show_keyboard, make_sound);
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
            if !append_buffer {
                running_buffer = String::from(string_buffer);
            } else {
                running_buffer.push_str(&string_buffer);
            }
            stdin.consume(length);
            _ = vrcclient.send_message(running_buffer.as_str(), !show_keyboard, make_sound);
        }
    }

    if typing_indicator {
        _ = vrcclient.typing_indicator(false);
    }
}

enum ParseArgsError {
    Int(ParseIntError),
    Addr(AddrParseError),
}

impl From<ParseIntError> for ParseArgsError {
    fn from(value: ParseIntError) -> Self {
        Self::Int(value)
    }
}
impl From<AddrParseError> for ParseArgsError {
    fn from(value: AddrParseError) -> Self {
        Self::Addr(value)
    }
}

impl Display for ParseArgsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseArgsError::Int(e) => f.write_fmt(format_args!("{}", e)),
            ParseArgsError::Addr(e) => f.write_fmt(format_args!("{}", e)),
        }
    }
}

#[allow(unused_assignments)] // line 142 gives this error but `i` is in fact used
fn parse_args() -> Result<ProgramSettings, (String, ParseArgsError)> {
    let mut settings = ProgramSettings::default();

    let args: Vec<String> = args().collect();
    for mut i in 0..args.len() {
        let arg = args[i].clone();
        match arg.as_str() {
            "--" => {
                settings.prompt = Some(args[i + 1..args.len()].join(" "));
                i = args.len();
            }
            "--help" => {
                settings.do_help = true;
            }
            "--version" => {
                settings.do_version = true;
            }
            "--client-port" => {
                i += 1;
                match args[i].parse::<u16>() {
                    Ok(v) => {
                        settings.client_port = v;
                    }
                    Err(e) => {
                        return Err((arg, ParseArgsError::from(e)));
                    }
                }
            }
            "--server-address" => {
                i += 1;
                match args[i].parse::<SocketAddr>() {
                    Ok(v) => {
                        settings.server_address = v;
                    }
                    Err(e) => {
                        return Err((arg, ParseArgsError::from(e)));
                    }
                }
            }
            "--enable-sfx" => {
                settings.make_sound = true;
            }
            "--show-keyboard" => {
                settings.show_keyboard = true;
            }
            "--append-mode" => {
                settings.append_buffer = true;
            }
            _ => {}
        }
    }

    Ok(settings)
}
#[warn(unused_assignments)]

fn help() {
    println!("Send input to VRChat chatbox
--help: show this message
--version: show a different message
--client-port: set port of osc client, or OS will choose randomly
--server-address: set address of vrchat osc server, or default to ({VRCHAT_OSC_ADDR})
--enable-sfx: every message sent makes the chatbox notification sound, probably don't enable
--show-keyboard: instead of instantly becoming a message, outputs it to client keyboard
--append-mode: if taking from stdin, add onto a growing buffer of a message instead of completely replacing the previous messages
--typing-indicator: enable in-game typing indicator for lifetime of this program");
}

fn version() {
    println!("{} - {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("{}", env!("CARGO_PKG_DESCRIPTION"));
    println!("hosted at {}", env!("CARGO_PKG_REPOSITORY"));
}
