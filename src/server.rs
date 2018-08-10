use std::net::SocketAddr;

use super::Application;

use tcpserver::*;

pub fn new<A: Application + Send + Sync + 'static>(listen_addr: SocketAddr, app: &'static A) {
    let mut server = TCPServer::new(app, listen_addr);
    server.serve().unwrap();
}