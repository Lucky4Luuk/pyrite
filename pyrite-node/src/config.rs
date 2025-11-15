use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub network: Network,
    pub compute: Compute,
}

#[derive(Serialize, Deserialize)]
pub struct Network {
    pub udp_port: u16,
    pub known_peers: Vec<SocketAddr>,
}

#[derive(Serialize, Deserialize)]
pub struct Compute {
    pub max_tasks: u8,
    pub allow_file_io: bool,
    pub allow_networking: bool,
}
