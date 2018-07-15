use Application;

use tokio_io::codec::{Decoder, Encoder};

use types::*;

use std::net::*;
use std::io;
use std::io::*;
use std::thread;
use std::sync::Mutex;
use std::ptr::Unique;

use integer_encoding::VarInt;
use byteorder::WriteBytesExt;
use bytes::{BigEndian, BytesMut, ByteOrder, BufMut};

use protobuf;
use protobuf::*;

const MAX_MESSAGE_SIZE:u64 = 104857600; // 100MB

pub struct TCPServer<A> where A: Application + 'static + Send + Sync{
    app: &'static A,
    addr: SocketAddr
}

impl<A: Application + 'static + Send + Sync> TCPServer<A> {
    pub fn new(app: &'static A, addr: SocketAddr) -> TCPServer<A> {
        TCPServer {
            app,
            addr
        }
    }
    pub fn serve(&mut self) -> io::Result<()> {
        let listener = TcpListener::bind(self.addr).unwrap();
        for new_connection in listener.incoming() {
            let app_ptr = Unique::new(self.app as *const A as *mut A).unwrap();
            match new_connection {
                Ok(stream) => {
                    println!("{:?}", stream);
                    thread::spawn( move || {
                        handle_stream(stream, app_ptr);
                    });
                },
                Err(err) => {
                    println!("Connection failed: {}", err);
                }
            }
        }
        drop(listener);
        Ok(())
    }
}

fn handle_stream(mut stream: TcpStream, app: Unique<Application>) {
    /*
    let mut connections = Mutex::new(Vec::new());
    // Create a new stream watcher thread
    let conn_thread = {
        let stream_pointer = Unique::new(&mut stream as *mut TcpStream).unwrap();
        let connections_pointer = Unique::new(&mut connections as *mut Mutex<Vec<Connection>>).unwrap();
        thread::spawn(move || {
            send_connections(stream_pointer, connections_pointer);
        })
    };
    */
    loop {
        let mut bytes = BytesMut::with_capacity(4096);
        bytes.put(&[0; 4096][..]);
        stream.read(bytes.as_mut());
        if bytes.len() == 0 {continue;}
        if bytes.as_ref() == [0; 4096].as_ref() {
            println!("abort!");
            break;
        }
        match decode(&mut bytes) {
            Ok(None) => {
                continue;
            }
            Ok(Some(req)) => {
                println!("Request: {:?}", req);
                respond(&mut stream, app.as_ptr(),req).unwrap();
            }
            Err(E) => {}
        }
    }
}

fn respond(stream: &mut TcpStream, app: *mut Application, request: Request) -> io::Result<()> {
    unsafe {
        let res: Response = {
            let app = &mut *(app);
            let mut response = Response::new();
            if request.has_info() {
                response.set_info(app.info(request.get_info()));
                response
            } else if request.has_set_option() {
                response.set_set_option(app.set_option(request.get_set_option()));
                response
            } else if request.has_query() {
                response.set_query(app.query(request.get_query()));
                response
            } else if request.has_check_tx() {
                response.set_check_tx(app.check_tx(request.get_check_tx()));
                response
            } else if request.has_init_chain() {
                response.set_init_chain(app.init_chain(request.get_init_chain()));
                response
            } else if request.has_begin_block() {
                response.set_begin_block(app.begin_block(request.get_begin_block()));
                response
            } else if request.has_deliver_tx() {
                response.set_deliver_tx(app.deliver_tx(request.get_deliver_tx()));
                response
            } else if request.has_end_block() {
                response.set_end_block(app.end_block(request.get_end_block()));
                response
            } else if request.has_commit() {
                response.set_commit(app.commit(request.get_commit()));
                response
            } else if request.has_echo() {
                response.set_echo(app.echo(request.get_echo()));
                response
            } else if request.has_flush() {
                response.set_flush(ResponseFlush::new());
                response
            } else {
                unreachable!();
            }
        };
        let mut buffer = BytesMut::with_capacity(4096);
        encode(res, &mut buffer);
        let mut buffer = buffer.into_iter().collect::<Vec<u8>>();
        println!("Writing on: {:?}", stream);
        stream.write(buffer.as_slice());
    }
    Ok(())
}

fn decode(buf: &mut BytesMut) -> io::Result<Option<Request>> {
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

fn encode(msg: Response, buf: &mut BytesMut) -> io::Result<()> {
    println!("Encoding {:?}", msg);
    let mut msg_to_vec = Vec::new();
    msg.write_to_vec(&mut msg_to_vec).unwrap();

    let msg_len: i64 = msg_to_vec.len() as i64;
    let varint = i64::encode_var_vec(msg_len);
    {
        let mut writer = buf.writer();

        writer.write(&varint).unwrap();
        writer.write(&msg_to_vec).unwrap();
        if !msg.has_flush() { writer.write(b"\x04\x1a\0").unwrap(); }
    }
    println!("{:?}", buf);
    Ok(())
}