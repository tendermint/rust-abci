#![cfg(feature="grpc_support")]

extern crate grpc;
extern crate rust_abci;


use rust_abci::grpc_server::new_server;
use rust_abci::types::*;
use rust_abci::types_grpc::*;

use std::thread;


struct DummyApp;

unsafe impl Sync for DummyApp {}

unsafe impl Send for DummyApp {}

impl ABCIApplication for DummyApp {
    fn echo(&self, o: ::grpc::RequestOptions, p: RequestEcho) -> ::grpc::SingleResponse<ResponseEcho> {
        println!("Echo");
        let response = ResponseEcho::new();
        ::grpc::SingleResponse::completed(response)
    }

    fn flush(&self, o: ::grpc::RequestOptions, p: RequestFlush) -> ::grpc::SingleResponse<ResponseFlush> {
        println!("Flush");
        let response = ResponseFlush::new();
        ::grpc::SingleResponse::completed(response)
    }

    fn info(&self, o: ::grpc::RequestOptions, p: RequestInfo) -> ::grpc::SingleResponse<ResponseInfo> {
        println!("Info");
        let response = ResponseInfo::new();
        ::grpc::SingleResponse::completed(response)
    }

    fn set_option(&self, o: ::grpc::RequestOptions, p: RequestSetOption) -> ::grpc::SingleResponse<ResponseSetOption> {
        println!("SetOption");
        let response = ResponseSetOption::new();
        ::grpc::SingleResponse::completed(response)
    }

    fn deliver_tx(&self, o: ::grpc::RequestOptions, p: RequestDeliverTx) -> ::grpc::SingleResponse<ResponseDeliverTx> {
        println!("DeliverTx");
        let response = ResponseDeliverTx::new();
        ::grpc::SingleResponse::completed(response)
    }

    fn check_tx(&self, o: ::grpc::RequestOptions, p: RequestCheckTx) -> ::grpc::SingleResponse<ResponseCheckTx> {
        println!("CheckTx");
        let response = ResponseCheckTx::new();
        ::grpc::SingleResponse::completed(response)
    }

    fn query(&self, o: ::grpc::RequestOptions, p: RequestQuery) -> ::grpc::SingleResponse<ResponseQuery> {
        println!("Query");
        let response = ResponseQuery::new();
        ::grpc::SingleResponse::completed(response)
    }

    fn commit(&self, o: ::grpc::RequestOptions, p: RequestCommit) -> ::grpc::SingleResponse<ResponseCommit> {
        println!("Commit");
        let response = ResponseCommit::new();
        ::grpc::SingleResponse::completed(response)
    }

    fn init_chain(&self, o: ::grpc::RequestOptions, p: RequestInitChain) -> ::grpc::SingleResponse<ResponseInitChain> {
        println!("InitChain");
        let response = ResponseInitChain::new();
        ::grpc::SingleResponse::completed(response)
    }

    fn begin_block(&self, o: ::grpc::RequestOptions, p: RequestBeginBlock) -> ::grpc::SingleResponse<ResponseBeginBlock> {
        println!("BeginBlock");
        let response = ResponseBeginBlock::new();
        ::grpc::SingleResponse::completed(response)
    }

    fn end_block(&self, o: ::grpc::RequestOptions, p: RequestEndBlock) -> ::grpc::SingleResponse<ResponseEndBlock> {
        println!("EndBlock");
        let response = ResponseEndBlock::new();
        ::grpc::SingleResponse::completed(response)
    }
}


fn main() {
    let listen_addr = "0.0.0.0:46658";

    let _server = new_server(listen_addr, DummyApp);

    loop {
        thread::park();
    }
}