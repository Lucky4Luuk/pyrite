//! This library defines an implementation of the pyrite network.

#[macro_use]
extern crate log;

use std::net::SocketAddr;

mod connection;

pub use connection::*;

pub struct PyriteNode {
    network_connection: NetworkConnection,
    known_peers: Vec<SocketAddr>,
}

impl PyriteNode {
    pub fn new(udp_port: u16, known_peers: Vec<SocketAddr>) -> anyhow::Result<Self> {
        let network_connection = NetworkConnection::new(udp_port)?;
        Ok(Self {
            network_connection,
            known_peers,
        })
    }

    pub fn start(&mut self) {
        // Jumpstart the network connection
        if let Err(e) = self
            .network_connection
            .jumpstart_discovery(&self.known_peers)
        {
            error!("Failed to start node discovery!");
            panic!("Failed to start node discovery! {e:?}");
        }

        info!("Node discovery started...");
    }

    pub fn process(&mut self) -> Option<NetworkMessage> {
        match self.network_connection.process() {
            Err(e) => {
                error!("{e}");
                None
            }
            Ok(m) => m,
        }
    }
}
