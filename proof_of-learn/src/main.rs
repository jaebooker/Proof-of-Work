#[macro_use]
extern crate serde_derive;
extern crate chrono;
mod blockchain;
use blockchain::*;
fn main() {
    println!("Welcome to P2P Rust Blockchain experiment");
}
#[cfg(test)]
mod tests {
    use super::*;
#[test]
    fn test_genesis_block() {
        //create blockchain
    let p2p_bc: Vec<Block> = vec![Block::genesis()];
    assert_eq!(p2p_bc[0].block_number , 1);
    assert_eq!(p2p_bc[0].transaction_list[0].transaction_details, "This is dummy transaction as genesis block has no transactions");
    }
#[test]
    fn test_new_block() {
    let mut p2p_bc: Vec<Block> = vec![Block::genesis()];
let new_txn = Transaction {
        transaction_id: String::from("1"),
        transaction_timestamp: 0,
        transaction_details: String::from("Testing a new transaction"),
    };
    let mut new_block = Block::new(vec![new_txn], &p2p_bc[p2p_bc.len() - 1]);
Block::mine_new_block(&mut new_block, &PREFIX);
    p2p_bc.push(new_block);
assert_eq!(p2p_bc.len(),2);
    assert_eq!(p2p_bc[1].transaction_list[0].transaction_details,"Testing a new transaction");
    }
}
