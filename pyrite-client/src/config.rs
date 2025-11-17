use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub network: Network,
}

#[derive(Serialize, Deserialize)]
pub struct Network {
    pub udp_port: u16,
    pub known_peers: Vec<SocketAddr>,
}
