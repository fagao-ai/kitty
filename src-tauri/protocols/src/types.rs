use std::net::SocketAddr;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct NodeInfo {
    pub socket_addr: SocketAddr,
    pub node_number: i8,
}