use std::net::SocketAddr;

use super::Application;

use tcpserver::*;

pub fn new<A>(listen_addr: SocketAddr, app: A)
where
    A: Application + 'static + Send + Sync,
{
    let server = TCPServer::new(app, listen_addr);
    server.serve().unwrap();
}
