extern crate tsp;
use tsp::*;
use tsp::types::*;

struct EchoApp;

impl Application for EchoApp {

    // Info/Query connection
    fn info(&mut self, req: &RequestInfo) -> ResponseInfo {
        ResponseInfo::new()
    }

    fn set_option(&mut self, req: &RequestSetOption) -> ResponseSetOption {
        ResponseSetOption::new()
    }

    fn query(&mut self, p: &RequestQuery) -> ResponseQuery {
        ResponseQuery::new()
    }

    // Mempool connection
    fn check_tx(&mut self, p: &RequestCheckTx) -> ResponseCheckTx {
        ResponseCheckTx::new()
    }

    // Consensus connection
    fn init_chain(&mut self, p: &RequestInitChain) -> ResponseInitChain {
        ResponseInitChain::new()
    }

    fn begin_block(&mut self, p: &RequestBeginBlock) -> ResponseBeginBlock {
        ResponseBeginBlock::new()
    }

    fn deliver_tx(&mut self, p: &RequestDeliverTx) -> ResponseDeliverTx {
        ResponseDeliverTx::new()
    }

    fn end_block(&mut self, p: &RequestEndBlock) -> ResponseEndBlock {
        ResponseEndBlock::new()
    }

    fn commit(&mut self, p: &RequestCommit) -> ResponseCommit {
        ResponseCommit::new()
    }

    // Miscellaneous connection
    fn echo(&mut self, p: &RequestEcho) -> ResponseEcho {
        ResponseEcho::new()
    }

    fn flush(&mut self, p: &RequestFlush) -> ResponseFlush {
        println!("Calling flush");
        ResponseFlush::new()
    }
}

fn main() {
    static APP: EchoApp = EchoApp;
    let addr = "127.0.0.1:46658".parse().unwrap();
    server::new(addr, &APP);
}