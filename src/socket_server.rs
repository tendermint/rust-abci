use types_grpc::ABCIApplication;
use super::Service;
use websocket::sync::Server;
use websocket::server::NoTlsAcceptor;


pub struct SocketServer<H> {
    server: Server<NoTlsAcceptor>,
    app: H,
}

impl<H: ABCIApplication + 'static + Sync + Send + 'static> Service for SocketServer<H> {}

pub fn new_server<H: ABCIApplication + 'static + Sync + Send + 'static>(listen_addr: &str, app: H) -> Box<Service> {
    unimplemented!();
}
