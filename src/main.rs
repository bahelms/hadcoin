mod blockchain;

use blockchain::Blockchain;

fn main() {
    let mut chain = Blockchain::new();
    chain.mine_block();
    chain.mine_block();
    chain.mine_block();
    println!("{:?}", chain);
    println!("Valid?: {}", chain.is_valid());
}
