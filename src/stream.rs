// Stream consumer and producer for ABCI Messages
use integer_encoding::VarInt;
use messages::abci::*;
use mockstream::SharedMockStream;
use protobuf::Message;
use std::fmt::{self, Debug, Formatter};
use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;

pub enum StreamWrapper {
    Mocked(SharedMockStream),
    Tcp(TcpStream),
}

impl io::Read for StreamWrapper {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match *self {
            StreamWrapper::Mocked(ref mut s) => s.read(buf),
            StreamWrapper::Tcp(ref mut s) => s.read(buf),
        }
    }
}

impl io::Write for StreamWrapper {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match *self {
            StreamWrapper::Mocked(ref mut s) => s.write(buf),
            StreamWrapper::Tcp(ref mut s) => s.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match *self {
            StreamWrapper::Mocked(ref mut s) => s.flush(),
            StreamWrapper::Tcp(ref mut s) => s.flush(),
        }
    }
}

pub struct AbciStream {
    stream: StreamWrapper,
}

impl Debug for AbciStream {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.stream {
            StreamWrapper::Mocked(_) => Ok(f.debug_struct("SharedMockStream").finish()?),
            StreamWrapper::Tcp(ref s) => s.fmt(f),
        }
    }
}

impl AbciStream {
    pub fn new(stream: StreamWrapper) -> AbciStream {
        AbciStream { stream }
    }

    /// Write an ABCI response to stream
    pub fn write_response(&mut self, response: &Response) -> io::Result<()> {
        let msg_size = response.compute_size() as usize;
        let varint = i64::encode_var_vec(msg_size as i64);
        let mut output = Vec::<u8>::with_capacity((msg_size + varint.len()) as usize);

        output.write_all(varint.as_slice())?;
        response.write_to_vec(&mut output)?;

        self.stream.write_all(output.as_slice())?;
        self.stream.flush()?;
        Ok(())
    }

    pub fn read_request(&mut self) -> Option<Request> {
        if let Ok(amount) = read_varint(&mut self.stream) {
            let mut buf = vec![0; amount as usize];
            return match self.stream.read_exact(&mut buf) {
                Ok(()) => protobuf::parse_from_bytes(buf.as_slice()).ok(),
                Err(_) => None,
            };
        }
        None
    }
}

// Adapted from integer-encoding...
fn read_varint(stream: &mut Read) -> Result<i64, io::Error> {
    const BUFLEN: usize = 10;
    let mut buf = [0 as u8; BUFLEN];
    let mut i = 0;

    loop {
        if i >= BUFLEN {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Unterminated varint",
            ));
        }
        let read = try!(stream.read(&mut buf[i..i + 1]));

        // EOF
        if read == 0 && i == 0 {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Reached EOF"));
        }
        if buf[i] & 0b10000000 == 0 {
            break;
        }
        i += 1;
    }
    let (result, _) = i64::decode_var(&buf[0..i + 1]);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    //use protobuf::Message;

    #[test]
    fn should_decode() {
        let mut mockstream = SharedMockStream::new();
        let mut msg_to_vec = Vec::new();

        let mut r = Request::new();
        let mut echo = RequestEcho::new();
        echo.set_message(String::from("Helloworld"));
        r.set_echo(echo);
        r.write_to_vec(&mut msg_to_vec).unwrap();

        let msg_len = msg_to_vec.len() as i64;
        let varint = i64::encode_var_vec(msg_len);

        mockstream.push_bytes_to_read(varint.as_slice());
        mockstream.push_bytes_to_read(msg_to_vec.as_slice());

        let wrapped = StreamWrapper::Mocked(mockstream.clone());
        let mut consumer = AbciStream::new(wrapped);

        // Produce should be consumer.write_all() which calls the AbciSteam write

        let packet = consumer.read_request();
        assert_eq!(packet.is_some(), true);
        let v = packet.unwrap();
        assert_eq!(v.has_echo(), true);
        assert_eq!(v.get_echo().get_message(), "Helloworld");
    }

    #[test]
    fn should_decode_large_request() {
        let mut mockstream = SharedMockStream::new();
        let mut msg_to_vec = Vec::new();

        let mut r = Request::new();
        let mut echo = RequestEcho::new();
        let st = (0..2 * 4096).map(|_| "X").collect::<String>();
        echo.set_message(st);
        r.set_echo(echo);
        r.write_to_vec(&mut msg_to_vec).unwrap();

        let msg_len = msg_to_vec.len() as i64;
        let varint = i64::encode_var_vec(msg_len);

        mockstream.push_bytes_to_read(varint.as_slice());
        mockstream.push_bytes_to_read(msg_to_vec.as_slice());

        let wrapped = StreamWrapper::Mocked(mockstream.clone());
        let mut consumer = AbciStream::new(wrapped);

        // Produce should be consumer.write_all() which calls the AbciSteam write

        let packet = consumer.read_request();
        assert_eq!(packet.is_some(), true);
        let v = packet.unwrap();
        assert_eq!(v.has_echo(), true);
    }

    #[test]
    fn should_encode() {
        let mockstream = SharedMockStream::new();
        let wrapped = StreamWrapper::Mocked(mockstream.clone());
        let mut stream = AbciStream::new(wrapped);

        let mut r = Response::new();
        let mut echo = ResponseEcho::new();
        echo.set_message(String::from("Helloworld"));
        r.set_echo(echo);

        assert_eq!(stream.write_response(&r).is_ok(), true);
    }
}
