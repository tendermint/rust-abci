extern crate futures;
extern crate futures_cpupool;
extern crate grpc;
extern crate protobuf;
extern crate tls_api;


pub mod types;
pub mod types_grpc;


use futures_cpupool::CpuPool;
use types_grpc::{ABCIApplication, ABCIApplicationServer};


// Container trait so that we can return either a GRPC or SOCKET server
pub trait Service {}

impl Service for ABCIApplicationServer {}


pub fn new_server<H: ABCIApplication + 'static + Sync + Send + 'static>(listen_addr: &str, connection_type: &str, app: H) -> Option<Box<Service>> {
    match connection_type {
        "grpc" => Some(Box::new(ABCIApplicationServer::new_pool(listen_addr, Default::default(), app, CpuPool::new(4)))),
        "socket" => None,
        _ => None
    }
}
