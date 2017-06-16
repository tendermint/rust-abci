use tokio_core::reactor::{Handle, Core};
use types_grpc::ABCIApplication;
use super::Service;
use websocket::sync::Server;
use websocket::message::{Message, OwnedMessage};
use websocket::server::NoTlsAcceptor;
use std::fmt::Debug;
use std::thread;

pub struct SocketServer<H> {
    server: Server<NoTlsAcceptor>,
    app: H,
}

// TODO
// - return a reference to the SocketServer
// - figure out why the connection are dropped in the for loop
// - figure out how to decode a message once it has arrived
// - call the app and return the result

pub struct Dummy {}

impl Service for Dummy {}

impl<H: ABCIApplication + 'static + Sync + Send + 'static> Service for SocketServer<H> {}

pub fn new_server<H: ABCIApplication + 'static + Sync + Send + 'static>(listen_addr: &str, app: H) -> Box<Service> {
    let mut server = SocketServer::new(listen_addr, app);
    server.start();
    return Box::new(Dummy{})
}

impl<H: ABCIApplication + 'static + Sync + Send + 'static> SocketServer<H> {
    fn new(listen_addr: &str, app: H) -> SocketServer<H> {
        let server = Server::bind(listen_addr).unwrap();

        SocketServer{
            server: server,
            app: app,
        }
    }

    fn start(mut self) {
        for request in self.server.filter_map(Result::ok) {
            // Spawn a new thread for each connection.
            println!("test2");
            thread::spawn(move || {
                println!("test3");
                let mut client = request.use_protocol("rust-websocket").accept().unwrap();
                let ip = client.peer_addr().unwrap();

                println!("Connection from {}", ip);

                let (mut receiver, mut sender) = client.split().unwrap();

                for message in receiver.incoming_messages() {
                    let message = message.unwrap();
                    println!("{:?}", &message);

                    match message {
                        OwnedMessage::Close(_) => {
                            let message = OwnedMessage::Close(None);
                            sender.send_message(&message).unwrap();
                            println!("Client {} disconnected", ip);
                            return;
                        }
                        OwnedMessage::Ping(ping) => {
                            let message = OwnedMessage::Pong(ping);
                            sender.send_message(&message).unwrap();
                        }
                        _ => sender.send_message(&message).unwrap(),
                    }
                }
            });
        }
    }
}