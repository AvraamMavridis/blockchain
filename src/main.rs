mod block;
mod blockchain;

fn main() {
    let mut chain = blockchain::BlockChain::new("./chain.json".to_string());

    chain.add_block("block 2".to_string());
    chain.add_block("block 3".to_string());

    if chain.validate_chain() {
        println!("Valid chain!");
    };
}
