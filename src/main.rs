// Create a CLI app in Rust that lets you add blocks, print the chain, and stores data on disk.

use tokio::net::TcpListener;
use std::{ io::{self, Write}};
pub mod block;
use block::Block;

use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main(){
    let listener = TcpListener::bind("127.0.0.1:3000").await.expect("Failed binding the server");
    println!("SERVER RUNNING AT 127.0.0.1:3000");
    let mut blockchain_json = tokio::fs::File::open("blockchain.json").await.unwrap();
    let mut user_input = String::new();
    let mut block_data = String::new();
    let mut blockchain: Vec<Block> = Vec::new();
    let mut buffer = String::new();

    blockchain_json.read_to_string(&mut buffer).await.unwrap();
    loop {
        if blockchain.is_empty() && buffer.len() > 0{
            let my_data:Vec<Block> = serde_json::from_str(&buffer).unwrap();
            for data in my_data{
                blockchain.push(data);
            }  
        }
        let socket_chain = serde_json::to_vec(&blockchain).unwrap();
        block_data.clear();
        user_input.clear();
        let (mut socket , addr) = listener.accept().await.expect("Failed connecting to a socket");
        println!("Connected to peer addr: {}" , addr);
        tokio::spawn(async move {
            socket.write_all(&socket_chain).await.expect("ERR in sending chain to a peer");
        });
        print!(" >>>>>> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut user_input).unwrap();

        if user_input.trim() == "ADD"{
            if let None = blockchain.last(){
                print!("What data you want to enter in block: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut block_data).unwrap();
                let genesis_block = Block::new(block_data.clone());
                blockchain.push(genesis_block);
                
            }else{
                print!("What data you want to enter in block: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut block_data).unwrap();
                let index: u64 = (blockchain.len()+1).try_into().unwrap();
                let blocks = Block::next(index, block_data.clone(), blockchain[blockchain.len()-1].current_hash.clone());
                blockchain.push(blocks);
            }}
        if user_input.trim() == "SHOW"{
            println!("{:#?}" , blockchain);
        }
        if user_input.trim() == "QUIT"{
            let mut file = tokio::fs::OpenOptions::new()
                                                            .truncate(true)
                                                            .create(true)
                                                            .read(true)
                                                            .write(true)
                                                            .open("blockchain.json").await.unwrap();
            let blockchain_json = serde_json::to_string_pretty(&blockchain).unwrap();
            let blockchain_data = format!("{}" , blockchain_json);
            file.write_all(blockchain_data.as_bytes()).await.unwrap();
            break
        }
        if user_input.trim() == "CHECK"{
            let checked_result = is_chain_valid(&mut blockchain);
            println!("IS the blockchain valid: {}" , checked_result);
        }
    }
}


fn is_chain_valid(chain : &mut Vec<Block>)->bool{
    let mut  checks = Vec::new();
    for (index , _block) in chain.iter().enumerate(){
        if index == 0{
            continue;
        }else{
            if index < chain.len()-1{
                let hash = chain[index+1].previous_hash.clone();
                if chain[index].current_hash == hash{ 
                    if chain[index].calculate_hash() == chain[index].current_hash{
                        checks.push(true);
                    }
                }else{
                    checks.push(false);
                }
            }else{
                if chain[index].calculate_hash() == chain[index].current_hash{
                    checks.push(true);
                }else {
                    checks.push(false);
                }
            }
        }
    }
            
    if checks.contains(&false){
        return false;
    }else{
        return true;
    }
}