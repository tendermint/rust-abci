extern crate protobuf;

pub mod abci; // Core types
pub mod merkle;
pub mod types; // Common (KV Pairs)

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
