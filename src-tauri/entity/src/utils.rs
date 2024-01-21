use rand::prelude::*;
use std::collections::HashSet;
use std::net::TcpListener;

const START_PORT: u16 = 20000;
const END_PORT: u16 = 30000;

fn is_port_available(port: u16) -> bool {
    if let Ok(listener) = TcpListener::bind(("127.0.0.1", port)) {
        drop(listener);
        return true;
    }
    false
}

pub fn get_random_port(used_ports: &HashSet<u16>) -> Option<u16> {
    let mut rng = thread_rng();
    const MAX_ATTEMPTS: u16 = 1000;

    for _ in 0..MAX_ATTEMPTS {
        let port = rng.gen_range(START_PORT..=END_PORT);
        if !used_ports.contains(&port) && is_port_available(port) {
            return Some(port);
        }
    }

    None
}