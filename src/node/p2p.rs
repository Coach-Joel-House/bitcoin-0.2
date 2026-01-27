use std::collections::HashMap;
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH, Duration};

use crate::core::block::Block;
use crate::core::transaction::Transaction;
use crate::core::chain::Blockchain;
use crate::validation::validate_transaction;
use crate::node::message::{NetworkMessage, PROTOCOL_VERSION};

const MAX_MESSAGE_SIZE: usize = 1 * 1024 * 1024;
const MAX_ADDRS: usize = 32;

pub struct PeerNode {
    pub address: SocketAddr,
    pub last_seen: i64,
    pub stream: TcpStream,
}

pub struct P2PNetwork {
    peers: Arc<Mutex<HashMap<String, PeerNode>>>,
    listener_addr: SocketAddr,
    chain: Arc<Mutex<Blockchain>>,
}

impl P2PNetwork {
    /// RANDOM PORT (non-seed)
    pub fn new(chain: Arc<Mutex<Blockchain>>) -> Self {
        Self::bind_internal("0.0.0.0:0", chain)
    }

    /// FIXED PORT (SEED)
    pub fn bind(bind_addr: &str, chain: Arc<Mutex<Blockchain>>) -> Self {
        Self::bind_internal(bind_addr, chain)
    }

    fn bind_internal(bind_addr: &str, chain: Arc<Mutex<Blockchain>>) -> Self {
        let listener = TcpListener::bind(bind_addr)
            .expect("P2P bind failed");
        listener.set_nonblocking(true).unwrap();

        let addr = listener.local_addr().unwrap();
        let peers = Arc::new(Mutex::new(HashMap::new()));

        let peers_accept = Arc::clone(&peers);
        let chain_accept = Arc::clone(&chain);

        thread::spawn(move || loop {
            match listener.accept() {
                Ok((stream, peer_addr)) => {
                    if peer_addr.ip().is_loopback() {
                        continue;
                    }

                    stream.set_read_timeout(Some(Duration::from_secs(30))).ok();

                    peers_accept.lock().unwrap().insert(
                        peer_addr.to_string(),
                        PeerNode {
                            address: peer_addr,
                            last_seen: now(),
                            stream: stream.try_clone().unwrap(),
                        },
                    );

                    let peers_inner = Arc::clone(&peers_accept);
                    let chain_inner = Arc::clone(&chain_accept);

                    thread::spawn(move || {
                        Self::handle_peer(stream, peers_inner, chain_inner);
                    });
                }
                Err(_) => thread::sleep(Duration::from_millis(100)),
            }
        });

        Self { peers, listener_addr: addr, chain }
    }

    pub fn connect(&self, addr: SocketAddr) {
        if addr.ip().is_loopback() {
            return;
        }

        if self.peers.lock().unwrap().contains_key(&addr.to_string()) {
            return;
        }

        if let Ok(mut stream) = TcpStream::connect(addr) {
            stream.set_read_timeout(Some(Duration::from_secs(30))).ok();

            let height = self.chain.lock().unwrap().height();
            let hello = NetworkMessage::Hello {
                version: PROTOCOL_VERSION,
                height,
                agent: "bitcoin-revelation-seed".to_string(),
            };

            let _ = stream.write_all(&bincode::serialize(&hello).unwrap());
            let _ = stream.write_all(&bincode::serialize(&NetworkMessage::GetAddr).unwrap());

            self.peers.lock().unwrap().insert(
                addr.to_string(),
                PeerNode {
                    address: addr,
                    last_seen: now(),
                    stream: stream.try_clone().unwrap(),
                },
            );

            let peers = Arc::clone(&self.peers);
            let chain = Arc::clone(&self.chain);

            thread::spawn(move || {
                Self::handle_peer(stream, peers, chain);
            });
        }
    }

    fn handle_peer(
        mut stream: TcpStream,
        peers: Arc<Mutex<HashMap<String, PeerNode>>>,
        chain: Arc<Mutex<Blockchain>>,
    ) {
        loop {
            let mut buf = vec![0u8; MAX_MESSAGE_SIZE];
            let n = match stream.read(&mut buf) {
                Ok(0) | Err(_) => break,
                Ok(n) => n,
            };

            let msg: NetworkMessage = match bincode::deserialize(&buf[..n]) {
                Ok(m) => m,
                Err(_) => continue,
            };

            match msg {
                NetworkMessage::GetAddr => {
                    let list: Vec<String> = peers.lock().unwrap()
                        .keys()
                        .take(MAX_ADDRS)
                        .cloned()
                        .collect();

                    let _ = stream.write_all(
                        &bincode::serialize(&NetworkMessage::Addr(list)).unwrap()
                    );
                }

                NetworkMessage::Block(b) => {
                    let _ = chain.lock().unwrap().validate_and_add_block(b);
                }

                NetworkMessage::Transaction(tx) => {
                    let c = chain.lock().unwrap();
                    let _ = validate_transaction(&tx, &c.utxos, c.height());
                }

                NetworkMessage::Ping => {
                    let _ = stream.write_all(
                        &bincode::serialize(&NetworkMessage::Pong).unwrap()
                    );
                }

                _ => {}
            }
        }

        if let Ok(addr) = stream.peer_addr() {
            peers.lock().unwrap().remove(&addr.to_string());
        }
    }

    pub fn peer_count(&self) -> usize {
        self.peers.lock().unwrap().len()
    }

    pub fn local_addr(&self) -> SocketAddr {
        self.listener_addr
    }
}

fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}
