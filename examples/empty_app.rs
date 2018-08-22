extern crate abci;

struct EmptyApp;

impl abci::Application for EmptyApp {
    fn deliver_tx(&mut self, _p: &abci::RequestDeliverTx) -> abci::ResponseDeliverTx {
        abci::ResponseDeliverTx::new()
    }
}

fn main() {
    let addr = "127.0.0.1:26658".parse().unwrap();
    abci::run(addr, EmptyApp);
}
