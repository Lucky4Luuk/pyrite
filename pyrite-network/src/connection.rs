use std::{
    collections::HashMap,
    io::{Error as IoError, ErrorKind as IoErrorKind},
    net::{SocketAddr, UdpSocket},
    time::{Duration, Instant},
};

use ciborium::de::Error;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RecvMessageError {
    #[error("io error {0:?}")]
    IoError(IoError),
    #[error("ciborium error {0:?}")]
    CiboriumError(Error<IoError>),
}

#[derive(Serialize, Deserialize)]
pub enum NetworkMessage {
    KeepAlive,
    NewNode { addr: SocketAddr },
    RequestPeerList,
    PeerList { peers: Vec<SocketAddr> },
}

pub struct NetworkConnection {
    socket: UdpSocket,
    read_buf: [u8; 65535],
    last_keep_alive: Instant,

    peers: HashMap<SocketAddr, Instant>,
}

impl NetworkConnection {
    pub fn new(port: u16) -> anyhow::Result<Self> {
        let socket = UdpSocket::bind(("0.0.0.0", port))?;
        info!("Addr: {:?}", socket.local_addr());
        socket.set_read_timeout(Some(Duration::from_secs(30)))?;

        Ok(Self {
            socket,
            read_buf: [0u8; 65535],
            last_keep_alive: Instant::now(),

            peers: HashMap::new(),
        })
    }

    pub fn jumpstart_discovery(&mut self, known_peers: &Vec<SocketAddr>) -> anyhow::Result<()> {
        for addr in known_peers {
            self.peers.insert(*addr, Instant::now());
        }
        self.broadcast(&NetworkMessage::RequestPeerList)?;
        Ok(())
    }

    pub fn recv_message(&mut self) -> Result<(NetworkMessage, SocketAddr), RecvMessageError> {
        let (bytes_read, addr) = self
            .socket
            .recv_from(&mut self.read_buf)
            .map_err(|e| RecvMessageError::IoError(e))?;
        let msg: NetworkMessage = ciborium::from_reader(&self.read_buf[..bytes_read])
            .map_err(|e| RecvMessageError::CiboriumError(e))?;
        Ok((msg, addr))
    }

    fn send_bytes_to(&mut self, bytes: &[u8], addr: SocketAddr) {
        match self.socket.send_to(bytes, addr) {
            Ok(n) => debug!("bytes_sent: {n}"),
            Err(e) => {
                error!(
                    "Failed to send message to {addr}! Removing from list of known peers. Error: {e}"
                );
                self.peers.remove(&addr);
            }
        }
    }

    pub fn send_message_to(
        &mut self,
        msg: &NetworkMessage,
        addr: SocketAddr,
    ) -> anyhow::Result<()> {
        let mut bytes = Vec::new();
        ciborium::into_writer(msg, &mut bytes)?;
        self.send_bytes_to(&bytes, addr);
        Ok(())
    }

    pub fn broadcast(&mut self, msg: &NetworkMessage) -> anyhow::Result<()> {
        let mut bytes = Vec::new();
        ciborium::into_writer(msg, &mut bytes)?;
        for (addr, _) in self.peers.clone() {
            self.send_bytes_to(&bytes, addr);
        }
        Ok(())
    }

    pub fn send_keep_alive(&mut self) -> anyhow::Result<()> {
        self.broadcast(&NetworkMessage::KeepAlive)?;
        info!("Keep alive sent!");
        self.last_keep_alive = Instant::now();
        Ok(())
    }

    pub fn process(&mut self) -> anyhow::Result<()> {
        if self.last_keep_alive.elapsed().as_secs() > 30 {
            self.send_keep_alive()?;
        }

        match self.recv_message() {
            Ok((msg, addr)) => {
                if let Some(keep_alive_time) = self.peers.get_mut(&addr) {
                    *keep_alive_time = Instant::now();
                } else {
                    if let Err(e) = self.broadcast(&NetworkMessage::NewNode { addr }) {
                        error!("Failed to broadcast message: {e:?}");
                    }
                    self.peers.insert(addr, Instant::now());
                }

                match msg {
                    NetworkMessage::NewNode { addr } => {
                        self.peers.insert(addr, Instant::now());
                        info!("New node! Current peers:\n{:#?}", self.peers);
                    }
                    NetworkMessage::RequestPeerList => {
                        info!("Client requested peer list!");
                        self.send_message_to(
                            &NetworkMessage::PeerList {
                                peers: self.peers.iter().map(|(addr, _)| *addr).collect::<Vec<_>>(),
                            },
                            addr,
                        )?;
                    }
                    NetworkMessage::PeerList { peers } => {
                        info!("Peers: {:?}", peers);
                    }

                    NetworkMessage::KeepAlive => {} // Ignore any keep alive packets, any packet updates keep alive
                    _ => error!("Unsupported network message!"),
                }
            }
            Err(e) => match e {
                RecvMessageError::IoError(e) if e.kind() == IoErrorKind::TimedOut => {
                    // Timed out! Time to send keep alive packet :3
                    self.send_keep_alive()?;
                }
                e => {
                    error!("Error: {e:?}");
                    return Err(e.into());
                }
            },
        }

        Ok(())
    }
}
