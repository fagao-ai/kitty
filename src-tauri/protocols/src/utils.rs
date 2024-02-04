use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

pub fn socket_addr_busy(socket_addr: SocketAddr) -> bool {
    let timeout_duration = Duration::from_secs(1);
    return TcpStream::connect_timeout(&socket_addr, timeout_duration).is_ok();
}
