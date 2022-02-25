use super::{hash, Blockchain};

#[test]
fn new_blockchain_creates_a_genesis_block() {
    let chain = Blockchain::new();
    let genesis = &chain.chain[0];
    assert_eq!(genesis.previous_hash, "0");
    assert_ne!(genesis.nonce, 1);
}

#[test]
fn mine_block_adds_new_blocks_to_the_chain() {
    let mut chain = Blockchain::new();
    chain.mine_block();
    chain.mine_block();
    chain.mine_block();
    assert_eq!(chain.chain.len(), 4);
}

#[test]
fn blocks_are_correctly_linked_by_hashes() {
    let mut chain = Blockchain::new();
    chain.mine_block();
    chain.mine_block();
    chain.mine_block();
    assert_eq!(chain.chain[1].previous_hash, hash(&chain.chain[0]));
    assert_eq!(chain.chain[2].previous_hash, hash(&chain.chain[1]));
    assert_eq!(chain.chain[3].previous_hash, hash(&chain.chain[2]));
}

#[test]
fn the_blockchain_is_valid() {
    let mut chain = Blockchain::new();
    chain.mine_block();
    chain.mine_block();
    chain.mine_block();
    assert_eq!(chain.is_valid(), true);
}
