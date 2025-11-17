#[macro_use]
extern crate log;

use pyrite_network::*;

fn main() {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(log::LevelFilter::Info)
        .init();
    info!("Hello, client!");

    let node = PyriteNode::new(config.network.udp_port, config.network.known_peers)
        .expect("Failed to create node!");

    node.start();
}
