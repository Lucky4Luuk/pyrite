#[macro_use]
extern crate log;

use std::{
    sync::mpsc::{self, SyncSender},
    thread,
};

use pyrite_network::*;

mod config;
mod tui;

use config::*;
use tui::*;

enum NodeMessages {}

fn main() {
    let (tx, rx) = mpsc::sync_channel(5);

    thread::spawn(move || node_main(tx));
}

fn node_main(msg_tx: SyncSender<NodeMessages>) {
    let config: Config = toml::from_str(
        &std::fs::read_to_string("config.toml").expect("Failed to read config.toml!"),
    )
    .expect("Failed to parse config.toml!");

    let mut node = PyriteNode::new(config.network.udp_port, config.network.known_peers)
        .expect("Failed to create node!");

    node.start();

    loop {
        if let Some(msg) = node.process() {
            match msg {
                NetworkMessage::KeepAlive => info!("pong"),
                _ => {}
            }
        }
    }
}
