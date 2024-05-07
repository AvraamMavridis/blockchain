use std::env;

use blockchain::BlockChain;

mod block;
mod blockchain;

enum Action {
    CreateOrLoad(String),
    Delete(String),
    Validate(String),
    AddBlock(String, String)

}

fn parse_action(args: &[String]) -> Action {
    match args[1].as_str() {
        "create" => Action::CreateOrLoad(args[2].to_string()),
        "delete" => Action::Delete(args[2].to_string()),
        "validate" => Action::Validate(args[2].to_string()),
        "addblock" => Action::AddBlock(args[2].to_string(), args[3].to_string()),
        _ => panic!("Invalid action")
    }
} 

fn main() {
    let args: Vec<String> = env::args().collect();
    let action: Action = parse_action(&args);

    match action {
        Action::CreateOrLoad(path) => {
            if BlockChain::exist(&path) {
                eprintln!("Chain already exist in that path {}", path);
            } else {
                BlockChain::new(format!("{}.json", path));
            }
        },
        Action::Delete(path) => {
            BlockChain::delete(&path);
        },
        Action::Validate(path) => {
            if BlockChain::exist(&path) {
                let chain = blockchain::BlockChain::new(format!("{}.json", path));
                if chain.validate_chain() {
                    println!("Valid chain!");
                } else {
                    eprintln!("Invalid chain!");
                }
            } else {
                eprintln!("Chain does not exist in that path {}", path);
            }

        },
        Action::AddBlock(path, data) => {
            let mut chain = BlockChain::load(format!("{}.json", path));
            chain.add_block(data);
        }
    }
}
