extern crate byteorder;
extern crate grpc;
extern crate rust_abci;


use rust_abci::new_server;
use rust_abci::types::*;
use rust_abci::types_grpc::*;


use std::sync::Mutex;
use std::thread;


use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};


struct CounterApp {
    serial: Mutex<bool>,
    txCount: Mutex<u64>,
    hashCount: Mutex<u64>,
}

impl CounterApp {
    fn new(serial: bool) -> CounterApp {
        CounterApp {
            serial: Mutex::new(serial),
            txCount: Mutex::new(0),
            hashCount: Mutex::new(0),
        }
    }
}

impl ABCIApplication for CounterApp {
    fn echo(&self, o: ::grpc::RequestOptions, p: RequestEcho) -> ::grpc::SingleResponse<ResponseEcho> {
        let echo = p.get_message();
        let mut response = ResponseEcho::new();
        response.set_message(echo.to_owned());
        ::grpc::SingleResponse::completed(response)
    }

    fn flush(&self, o: ::grpc::RequestOptions, p: RequestFlush) -> ::grpc::SingleResponse<ResponseFlush> {
        unimplemented!();
    }

    fn info(&self, o: ::grpc::RequestOptions, p: RequestInfo) -> ::grpc::SingleResponse<ResponseInfo> {
        let mut response = ResponseInfo::new();
        response.set_data("CounterApp".to_owned());
        response.set_version("0.1.0".to_owned());
        ::grpc::SingleResponse::completed(response)
    }

    fn set_option(&self, o: ::grpc::RequestOptions, p: RequestSetOption) -> ::grpc::SingleResponse<ResponseSetOption> {
        let mut response = ResponseSetOption::new();
        if p.get_key() == "serial" && p.get_value() == "on" {
            let mut serial = self.serial.lock().unwrap();
            *serial = true;
            response.set_log("Serial set to ON".to_owned());
        }
        ::grpc::SingleResponse::completed(response)
    }

    fn deliver_tx(&self, o: ::grpc::RequestOptions, p: RequestDeliverTx) -> ::grpc::SingleResponse<ResponseDeliverTx> {
        let mut response = ResponseDeliverTx::new();
        if *self.serial.lock().unwrap() {
            if p.get_tx().len() > 8 {
                response.set_code(CodeType::EncodingError);
                response.set_log("Max tx size is 8 bytes".to_owned());
                return ::grpc::SingleResponse::completed(response);
            }
        }
        let nonce = p.get_tx().read_uint::<BigEndian>(p.get_tx().len()).unwrap();
        if *self.txCount.lock().unwrap() != nonce {
            response.set_code(CodeType::BadNonce);
            response.set_log("Invalid nonce.".to_owned());
            return ::grpc::SingleResponse::completed(response);
        }
        let mut txCount = self.txCount.lock().unwrap();
        *txCount += 1;
        response.set_code(CodeType::OK);
        ::grpc::SingleResponse::completed(response)
    }

    fn check_tx(&self, o: ::grpc::RequestOptions, p: RequestCheckTx) -> ::grpc::SingleResponse<ResponseCheckTx> {
        let mut response = ResponseCheckTx::new();
        if *self.serial.lock().unwrap() {
            if p.get_tx().len() > 8 {
                response.set_code(CodeType::EncodingError);
                response.set_log("Max tx size is 8 bytes".to_owned());
                return ::grpc::SingleResponse::completed(response);
            }
        }
        let nonce = p.get_tx().read_uint::<BigEndian>(p.get_tx().len()).unwrap();
        if *self.txCount.lock().unwrap() != nonce {
            response.set_code(CodeType::BadNonce);
            response.set_log("Invalid nonce.".to_owned());
            return ::grpc::SingleResponse::completed(response);
        }
        response.set_code(CodeType::OK);
        ::grpc::SingleResponse::completed(response)
    }

    fn query(&self, o: ::grpc::RequestOptions, p: RequestQuery) -> ::grpc::SingleResponse<ResponseQuery> {
        let mut response = ResponseQuery::new();
        match p.get_path() {
            "hash" => {
                let mut data = vec![];
                data.write_uint::<BigEndian>(*self.hashCount.lock().unwrap(), 8);
                response.set_value(data);
                return ::grpc::SingleResponse::completed(response);
            },
            "tx" => {
                let mut data = vec![];
                data.write_uint::<BigEndian>(*self.txCount.lock().unwrap(), 8);
                response.set_value(data);
                return ::grpc::SingleResponse::completed(response);
            },
            _ => {
                response.set_log("Invalid query path. Expected hash or tx.".to_owned());
                return ::grpc::SingleResponse::completed(response);
            },
        }
    }

    fn commit(&self, o: ::grpc::RequestOptions, p: RequestCommit) -> ::grpc::SingleResponse<ResponseCommit> {
        let mut response = ResponseCommit::new();

        let mut hashCount = self.hashCount.lock().unwrap();
        *hashCount += 1;

        if *self.txCount.lock().unwrap() == 0 {
            response.set_code(CodeType::OK);
            return ::grpc::SingleResponse::completed(response);
        }

        let mut data = vec![];
        data.write_uint::<BigEndian>(*self.txCount.lock().unwrap(), 8);
        response.set_data(data);
        ::grpc::SingleResponse::completed(response)
    }

    fn init_chain(&self, o: ::grpc::RequestOptions, p: RequestInitChain) -> ::grpc::SingleResponse<ResponseInitChain> {
        let response = ResponseInitChain::new();
        ::grpc::SingleResponse::completed(response)
    }

    fn begin_block(&self, o: ::grpc::RequestOptions, p: RequestBeginBlock) -> ::grpc::SingleResponse<ResponseBeginBlock> {
        let response = ResponseBeginBlock::new();
        ::grpc::SingleResponse::completed(response)
    }

    fn end_block(&self, o: ::grpc::RequestOptions, p: RequestEndBlock) -> ::grpc::SingleResponse<ResponseEndBlock> {
        let response = ResponseEndBlock::new();
        ::grpc::SingleResponse::completed(response)
    }
}

fn main() {
    let listen_addr = "0.0.0.0:46658";
    let connection_type = "grpc";

    let app = CounterApp::new(true);

    let _server = new_server(listen_addr, connection_type, app);

    loop {
        thread::park();
    }
}