//! Very simple VRChat OSC crate, only for the in-game chatbox (other osc endpoints coming 2050)
//!
//! Provides [Client] which can send "chatbox/input" or "chatbox/typing" messages to any address you set.
//!

use rosc::{encoder, OscMessage, OscPacket, OscType};
use std::{
    io,
    net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket},
};

/// The address VRChat listens for OSC messages on if not changed in the VRChat config.json.
pub const VRCHAT_OSC_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 9000);

/// Build a [Client] with the pattern everyone likes.
#[derive(Copy, Clone, Debug)]
pub struct ClientBuilder {
    client: SocketAddr,
    server: SocketAddr,
}

/// Contains the socket to connect to VRChat OSC to send messages.
///
/// Can either be created with a [ClientBuilder] or [Client::new]
#[derive(Debug)]
pub struct Client {
    client_socket: UdpSocket,
    server_address: SocketAddr,
}

impl Client {
    /// Create a [Client] without [ClientBuilder].
    /// client_socket.
    pub fn new(server_address: SocketAddr, client_socket: UdpSocket) -> Self {
        Client {
            client_socket,
            server_address,
        }
    }
}

impl ClientBuilder {
    /// Create a new [ClientBuilder].
    /// Without changes this will build as a [Client] with a socket, on a port chosen by the OS, connected to [VRCHAT_OSC_ADDR].
    pub fn new() -> Self {
        ClientBuilder {
            client: SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 0),
            server: VRCHAT_OSC_ADDR,
        }
    }

    pub fn with_server_ip(mut self, ip: IpAddr) -> ClientBuilder {
        self.server.set_ip(ip);
        self
    }

    pub fn with_server_port(mut self, port: u16) -> ClientBuilder {
        self.server.set_port(port);
        self
    }

    /// Port is already chosen by the OS if this is not used.
    pub fn with_client_port(mut self, port: u16) -> ClientBuilder {
        self.client.set_port(port);
        self
    }

    /// Build the [Client].
    /// Err when it cannot bind to the set port or connect to the set server address.
    pub fn build(&self) -> io::Result<Client> {
        let socket = UdpSocket::bind(self.client)?;
        socket.connect(self.server)?;
        Ok(Client {
            client_socket: socket,
            server_address: self.server,
        })
    }
}

impl Client {
    /// Change the address (ip and port) that the client sends OSC messages to.
    ///
    /// Err if the socket could not connect to the new address
    pub fn change_server_address(&mut self, new_address: SocketAddr) -> io::Result<()> {
        self.change_server_ip(new_address.ip())?;
        self.change_server_port(new_address.port())?;
        Ok(())
    }

    /// Change the ip that the client sends OSC messages to.
    ///
    /// Err if the socket could not connect to the new address.
    pub fn change_server_ip(&mut self, new_ip: IpAddr) -> io::Result<()> {
        self.server_address.set_ip(new_ip);
        self.client_socket.connect(self.server_address)?;
        Ok(())
    }

    /// Change the port that the client sends OSC messages to.
    ///
    /// Err if the socket could not connect to the new address.
    pub fn change_server_port(&mut self, new_port: u16) -> io::Result<()> {
        self.server_address.set_port(new_port);
        self.client_socket.connect(self.server_address)?;
        Ok(())
    }

    /// Set the visibility of the typing indicator.
    ///
    /// Err if the client is not connected to anything
    ///
    /// Panics if it could not encode an OscMessage (Should not happen, and if it does, not much a downstream user can do about it anyway)
    pub fn typing_indicator(&self, toggle: bool) -> io::Result<()> {
        match encoder::encode(&OscPacket::Message(OscMessage {
            addr: String::from("/chatbox/typing"),
            args: vec![OscType::Bool(toggle)],
        })) {
            Err(e) => {
                panic!("vrchat-box error when encoding osc message: {e}")
            }
            Ok(msg_buf) => {
                self.client_socket.send(&msg_buf)?;
            }
        }
        Ok(())
    }

    /// Send a chatbox message.
    ///
    /// Err if the client is not connected to anything.
    ///
    /// Panics if it could not encode an OscMessage (Should not happen, and if it does, not much a downstream user can do about it anyway)
    pub fn send_message(&self, msg: &str, keyboard: bool, sfx: bool) -> io::Result<()> {
        match encoder::encode(&OscPacket::Message(OscMessage {
            addr: String::from("/chatbox/input"),
            args: vec![
                OscType::String(msg.to_string()),
                OscType::Bool(keyboard),
                OscType::Bool(sfx),
            ],
        })) {
            Err(e) => {
                panic!("vrchat-box error when encoding osc message: {e}")
            }
            Ok(msg_buf) => {
                self.client_socket.send(&msg_buf)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_it_work() {
        let vrc_client = ClientBuilder::new()
            .with_server_ip(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))
            .with_server_port(9000)
            .build();
        assert!(vrc_client.is_ok());
    }

    #[test]
    fn does_it_send() {
        let server = UdpSocket::bind(SocketAddr::new(VRCHAT_OSC_ADDR.ip(), 0)).unwrap();
        let server_addr = server.local_addr().unwrap();
        server.set_nonblocking(true).unwrap();

        let client = ClientBuilder::new()
            .with_server_ip(server_addr.ip())
            .with_server_port(server_addr.port())
            .build()
            .unwrap();

        assert!(client
            .send_message("helloooooOO!!!!!", false, false)
            .is_ok());

        let mut buf = [0; 8];
        server.recv(&mut buf).unwrap();
    }
}
