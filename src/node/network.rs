use std::sync::{Arc, Mutex};
use std::net::SocketAddr;

use crate::core::block::Block;
use crate::core::transaction::Transaction;
use crate::core::chain::Blockchain;

use crate::node::p2p::P2PNetwork as Inner;

pub struct P2PNetwork {
    inner: Inner,
}

impl P2PNetwork {
    pub fn new(chain: Arc<Mutex<Blockchain>>) -> Self {
        Self { inner: Inner::new(chain) }
    }

    pub fn bind(addr: &str, chain: Arc<Mutex<Blockchain>>) -> Self {
        Self { inner: Inner::bind(addr, chain) }
    }

    pub fn connect(&self, addr: SocketAddr) {
        self.inner.connect(addr);
    }

    pub fn peer_count(&self) -> usize {
        self.inner.peer_count()
    }

    pub fn local_addr(&self) -> SocketAddr {
        self.inner.local_addr()
    }
}
