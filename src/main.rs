// Copyright © 2024 TooManyChoices
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use rosc::encoder;
use rosc::{OscMessage, OscPacket, OscType};
use std::env;
use std::io::{self, BufRead};
use std::net::{SocketAddrV4, UdpSocket};
use std::process::exit;
use std::str::{self, FromStr};

struct ProgramSettings {
    server_address: String,
    client_port: String,
    show_keyboard: bool,
    make_sound: bool,
    append_buffer: bool,
    prompt: String,
}

fn main() {
    let settings = parse_args();

    let client_address =
        SocketAddrV4::from_str(format!("127.0.0.1:{}", settings.client_port).as_str())
            .unwrap_or_else(|_| {
                println!("error parsing client ip (make sure -p arg is correct)");
                exit(1);
            });
    let socket = UdpSocket::bind(client_address).unwrap_or_else(|_| {
        println!("error binding to address (change port with -p flag)");
        exit(1);
    });
    let server_ip = SocketAddrV4::from_str(&settings.server_address).unwrap_or_else(|_| {
        println!("error parsing server address (make sure -d arg is valid)");
        exit(1);
    });

    if settings.prompt.as_str().chars().count() > 0 {
        chatbox_input(
            &socket,
            &server_ip,
            settings.prompt,
            !settings.show_keyboard,
            settings.make_sound,
        );
    } else {
        let mut stdin = io::stdin().lock();
        let mut running_buffer = String::new();
        loop {
            let buffer = match stdin.fill_buf() {
                Ok(v) => v,
                Err(_) => {
                    panic!()
                }
            };
            let string_buffer = str::from_utf8(buffer).unwrap();
            if !settings.append_buffer {
                running_buffer = String::from(string_buffer);
            } else {
                running_buffer.push_str(str::from_utf8(buffer).unwrap());
            }
            let length = buffer.len();
            if length == 0 {
                break;
            }
            stdin.consume(length);
            if length <= 0 {
                continue;
            }
            chatbox_input(
                &socket,
                &server_ip,
                running_buffer.clone(),
                !settings.show_keyboard,
                settings.make_sound,
            );
        }
    }

    exit(0);
}

fn chatbox_input(from: &UdpSocket, to: &SocketAddrV4, msg: String, keyboard: bool, sfx: bool) {
    match encoder::encode(&OscPacket::Message(OscMessage {
        addr: String::from("/chatbox/input"),
        args: vec![
            OscType::String(msg),
            OscType::Bool(keyboard),
            OscType::Bool(sfx),
        ],
    })) {
        Err(e) => {
            println!("error encoding message: {}", e);
        }
        Ok(msg_buf) => {
            from.send_to(&msg_buf, to).unwrap_or_else(|e| {
                println!("error when sending message: {}", e);
                return 0;
            });
        }
    }
}

fn parse_args() -> ProgramSettings {
    let mut settings = ProgramSettings {
        server_address: String::from("127.0.0.1:9000"),
        client_port: String::from("5236"),
        show_keyboard: false,
        make_sound: false,
        append_buffer: false,
        prompt: String::new(),
    };
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let mut it = args.iter();
        it.next();
        while let Some(arg) = it.next() {
            let mut charit = arg.chars();
            if charit.next().unwrap_or('\r') == '-' {
                for c in charit {
                    parse_flag(c, &mut settings, &mut it);
                }
            } else {
                println!("loose operand \"{arg}\" use \"--\" before operands");
                exit(1);
            }
        }
    }
    return settings;
}

fn parse_flag(c: char, settings: &mut ProgramSettings, it: &mut dyn Iterator<Item = &String>) {
    match c {
        '-' => {
            while let Some(operand) = it.next() {
                settings.prompt += operand;
            }
        }
        'p' => {
            settings.client_port = it
                .next()
                .unwrap_or_else(|| {
                    println!("expected arg for -p");
                    exit(1);
                })
                .clone();
        }
        'd' => {
            settings.server_address = it
                .next()
                .unwrap_or_else(|| {
                    println!("expected arg for -d");
                    exit(1);
                })
                .clone();
        }
        'k' => {
            settings.show_keyboard = true;
        }
        's' => {
            settings.make_sound = true;
        }
        'a' => {
            settings.append_buffer = true;
        }
        'h' => {
            help();
        }
        'H' => {
            Help();
        }
        'v' => {
            version();
        }
        _ => {
            println!("unknown flag -{}", c);
            exit(1);
        }
    }
}

fn help() {
    println!("send stdin/operands to vrchat as chatbox input");
    println!("usage: vrchatbox[-ksw][-p arg][-d arg]--[operand...]");
    println!("-h \t: show this");
    println!("-H \t: show this except proper");
    println!("-v \t: show version");
    println!("-p port : set osc client port (default 5236)");
    println!("-d addr : set server/vrchat osc address (default 127.0.0.1:9000)");
    println!("-k \t: if given, opens vrc keyboard instead");
    println!("-s \t: if given, enables chatbox notification sfx");
    println!("-a \t: add stdin to current chatbox input, for growing text instead of new text");
    exit(0);
}

#[allow(non_snake_case)]
fn Help() {
    println!("Send stdin or operands to VRChat as chatbox input.");
    println!("Usage: vrchatbox[-ksw][-p arg][-d arg]--[operand...]");
    println!("-h \t: Show this message and exit, but improper.");
    println!("-H \t: Show this message and exit.");
    println!("-v \t: Show the version and exit.");
    println!("-p port : Set the OSC client port, defaults to 5236");
    println!("-d addr : Set server (VRChat) address, defaults to 127.0.0.1:9000");
    println!("-k \t: Open ingame keyboard with input instead.");
    println!("-s \t: Enables chatbox notification sound to be triggered.");
    println!("-a \t: Append incoming stdin to current chatbox input, for growing text instead of replacing text.");
    exit(0);
}

fn version() {
    println!("{} - {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("{}", env!("CARGO_PKG_DESCRIPTION"));
    println!("hosted at {}", env!("CARGO_PKG_REPOSITORY"));
    exit(0);
}
