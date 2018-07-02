use std::cmp;
use std::io;
use std::io::Write;
use std::net::SocketAddr;

use types::*;
use super::Application;


use bytes::{BigEndian, BytesMut, ByteOrder, BufMut};

use byteorder::WriteBytesExt;

use futures::future;
use futures::{BoxFuture, Future};

use protobuf;
use protobuf::Message;

use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::{Decoder, Encoder, Framed};

use tokio_proto::TcpServer;
use tokio_proto::pipeline::ServerProto;

use tokio_service::Service;

use integer_encoding::{VarInt, VarIntReader, VarIntWriter};

pub fn new<A: Application + Send + Sync + 'static>(listen_addr: SocketAddr, app: &'static A) {
    let server = TcpServer::new(TSPProto, listen_addr);
    server.serve(move|| Ok(TSPService{app: app}));
}

// A codec describes how to go from a bunch of bytes from the wire into a
// deserialised request. The codec handles the deserialisation from buffer
// to request as defined in types.proto
struct TSPCodec;

const MAX_MESSAGE_SIZE:u64 = 104857600; // 100MB

impl Decoder for TSPCodec {
    type Item = Request;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<Request>> {
        let length = buf.len();

        if length == 0 || length > MAX_MESSAGE_SIZE as usize {
            return Ok(None);
        }

        let varint:(i64,usize) = i64::decode_var(&buf[..]);

        let message = protobuf::parse_from_bytes(
            &buf[varint.1 .. (varint.0 as usize + varint.1)]);

        buf.split_to(length);

        Ok(message.ok())
    }
}

impl Encoder for TSPCodec {
    type Item = Response;
    type Error = io::Error;

    fn encode(&mut self, msg: Response, buf: &mut BytesMut) -> io::Result<()> {
        let mut msg_to_vec = Vec::new();
        msg.write_to_vec(&mut msg_to_vec).unwrap();

        let msg_len: i64 = msg_to_vec.len() as i64;
        let varint = i64::encode_var_vec(msg_len);

        let mut writer = buf.writer();

        writer.write(&varint).unwrap();
        writer.write(&msg_to_vec).unwrap();
        writer.write(b"\x04\x1a\0").unwrap();

        Ok(())
    }

}

struct TSPProto;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for TSPProto {
    type Request = Request;
    type Response = Response;
    type Transport = Framed<T, TSPCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(TSPCodec))
    }
}


struct TSPService {
    app: &'static Application
}

impl Service for TSPService {
    type Request = Request;
    type Response = Response;
    type Error = io::Error;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let response = self.handle(&req);
        future::ok(response).boxed()
    }
}

impl TSPService {
    fn handle(&self, request: &Request) -> Response {
        unsafe {
            let app_ptr = self.app as *const Application;

            // Here we do some really trippy pointer magic
            let app_ptr = app_ptr as *mut Application;
            // Now we have a mutable pointer from a immutable reference. Unsafe Rust is epic.
            let app = &mut *app_ptr; 

        let mut response = Response::new();

        // Info/Query connection
        if request.has_info() {
            response.set_info(app.info(request.get_info()));
            return response;
        }

        if request.has_set_option() {
            response.set_set_option(app.set_option(request.get_set_option()));
            return response;
        }

        if request.has_query() {
            response.set_query(app.query(request.get_query()));
            return response;
        }

        // Mempool connection
        if request.has_check_tx() {
            response.set_check_tx(app.check_tx(request.get_check_tx()));
            return response;
        }

        // Consensus connection
        if request.has_init_chain() {
            response.set_init_chain(app.init_chain(request.get_init_chain()));
            return response;
        }

        if request.has_begin_block() {
            response.set_begin_block(app.begin_block(request.get_begin_block()));
            return response;
        }

        if request.has_deliver_tx() {
            response.set_deliver_tx(app.deliver_tx(request.get_deliver_tx()));
            return response;
        }

        if request.has_end_block() {
            response.set_end_block(app.end_block(request.get_end_block()));
            return response;
        }

        if request.has_commit() {
            response.set_commit(app.commit(request.get_commit()));
            return response;
        }

        // Miscellaneous connection
        if request.has_echo() {
            response.set_echo(app.echo(request.get_echo()));
            return response;
        }

        if request.has_flush() {
            response.set_flush(app.flush(request.get_flush()));
            return response;
        }

        unreachable!();
        }
    }
}
