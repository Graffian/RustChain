use std::io::{self, Write};

use tokio::{io::AsyncReadExt, net::TcpStream};
use blockchain::block::Block;


#[tokio::main]
async fn main(){
    let mut user_input = String::new();
    let mut blockchain = String::new();
    loop {
        user_input.clear();
        blockchain.clear();
        let mut stream = TcpStream::connect("127.0.0.1:3000").await.expect("Failed connecting to broadcast server");
        println!("PEER CONNECTED TO BROADCAST SERVER");
        print!(" >>>>>>>>> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut user_input).unwrap();
        stream.read_to_string(&mut blockchain).await.unwrap();
        if user_input.trim() == "SHOW"{
            println!("{:#?}" , blockchain);
            let chain_history : Vec<Block> = serde_json::from_str(&blockchain).unwrap();
            println!("{:#?}" , chain_history);
        }

        if user_input.trim() == "QUIT"{
            break;
        }
    }
}