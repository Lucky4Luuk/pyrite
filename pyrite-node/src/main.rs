#[macro_use]
extern crate log;

use pyrite_network::*;

mod config;
mod task;

use config::*;

fn main() {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    info!("Hello, world!");

    let config: Config = toml::from_str(
        &std::fs::read_to_string("config.toml").expect("Failed to read config.toml!"),
    )
    .expect("Failed to parse config.toml!");

    let node = PyriteNode::new(config.network.udp_port, config.network.known_peers)
        .expect("Failed to create node!");

    node.start();
}
