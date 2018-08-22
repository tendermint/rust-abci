use types::*;
use Application;

use std::io;
use std::io::*;
use std::net::*;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use std::thread;

use bytes::{BufMut, BytesMut};
use integer_encoding::VarInt;

use protobuf;
use protobuf::Message;

const BUFFER_SIZE: usize = 4096;

pub struct TCPServer<A>
where
    A: Application + 'static + Send + Sync,
{
    app: A,
    addr: SocketAddr,
}

impl<A> TCPServer<A>
where
    A: Application + 'static + Send + Sync,
{
    pub fn new(app: A, addr: SocketAddr) -> TCPServer<A> {
        TCPServer { app, addr }
    }

    pub fn serve(self) -> io::Result<()> {
        let listener = TcpListener::bind(self.addr).unwrap();

        let app = Arc::new(Mutex::new(self.app));

        for new_connection in listener.incoming() {
            let app_instance = Arc::clone(&app);

            match new_connection {
                Ok(stream) => {
                    thread::spawn(move || handle_stream(stream, app_instance));
                }
                Err(err) => {
                    println!("Connection failed: {}", err);
                }
            }
        }
        drop(listener);
        Ok(())
    }
}

fn handle_stream<A>(mut stream: TcpStream, app: Arc<Mutex<A>>)
where
    A: Application + 'static + Send + Sync,
{
    loop {
        let mut bytes = BytesMut::with_capacity(BUFFER_SIZE);
        bytes.put(&[0; BUFFER_SIZE][..]);
        stream.read(bytes.as_mut()).unwrap();
        if bytes.as_ref() == [0; BUFFER_SIZE].as_ref() {
            break;
        }

        for message in process_bytes(bytes) {
            let mut guard = app.lock().unwrap();
            let a = guard.deref_mut();
            respond(&mut stream, a, message).unwrap();
        }
    }
    println!("Connection closed on {:?}", stream);
}

fn respond<A>(stream: &mut TcpStream, app: &mut A, request: Request) -> io::Result<()>
where
    A: Application + 'static + Send + Sync,
{
    let res: Response = {
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
            let echo_msg = response.get_echo().get_message().to_string();
            response.set_echo({
                let mut echo = ResponseEcho::new();
                echo.set_message(echo_msg);
                echo
            });
            response
        } else if request.has_flush() {
            response.set_flush(ResponseFlush::new());
            response
        } else {
            unreachable!();
        }
    };

    let mut buffer = BytesMut::with_capacity(BUFFER_SIZE);
    encode(res, &mut buffer);
    let buffer = buffer.into_iter().collect::<Vec<u8>>();
    stream.write(buffer.as_slice()).unwrap();
    stream.flush().unwrap();

    Ok(())
}

fn process_bytes(mut bytes: BytesMut) -> Vec<Request> {
    let mut messages = Vec::<Request>::new();
    loop {
        if bytes[0] == 0 || bytes.len() == 0 {
            break;
        }
        let varint = i64::decode_var(&bytes[..]);
        let msg_bytes = bytes.split_to(varint.0 as usize + varint.1);
        match protobuf::parse_from_bytes(&msg_bytes[varint.1..]).ok() {
            Some(req) => {
                messages.push(req);
            }
            None => {
                println!("NOTE: Invalid request made");
            }
        }
    }
    messages
}

fn encode(msg: Response, buf: &mut BytesMut) {
    let mut msg_to_vec = Vec::new();
    msg.write_to_vec(&mut msg_to_vec).unwrap();
    let msg_len = msg_to_vec.len() as i64;
    let varint = i64::encode_var_vec(msg_len);
    {
        let mut writer = buf.writer();
        writer.write(&varint).unwrap();
        writer.write(&msg_to_vec).unwrap();
    }
}
