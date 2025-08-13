
use std::env;
use chrono::Utc;
use blockchain::block::Block;
use blockchain::transaction::Transaction;
use rand::rngs::OsRng;
use tokio::{io::AsyncReadExt, net::TcpStream};
#[tokio::main]
async fn main(){
    let mut csprng = OsRng{};
    let mut keypair = ed25519_dalek::Keypair::generate(&mut csprng);
    let now = Utc::now();
    let timestamp = now.timestamp();
    let args:Vec<String> = env::args().collect();
    let amount:Vec<char> = args[2].chars().collect();
    let sender = args[4].clone();
    let reciever = String::from("Bob");
    let amount=amount[0].to_digit(10).unwrap() as i32;
    let timestamp=timestamp;

    let signed_transaction = Transaction::sign_transaction(sender , reciever , amount , timestamp , &mut keypair);
    println!("The verified transaction is {:#?}" , signed_transaction);
    let public_key = keypair.public;
    let is_transaction_verified = Transaction::verify_transaction(&signed_transaction , &public_key);
    if is_transaction_verified == true{
        println!("transaction is verified");
    }else{
        println!("Trasaction is not verified");
    }
    let mut stream = TcpStream::connect("127.0.0.1:3000").await.expect("Failed connectng to server");
    let mut buffer = String::new();
    stream.read_to_string(&mut buffer).await.expect("Error reading blocks from server");
    let blockchain_vec:Vec<Block> = serde_json::from_str(&buffer).unwrap();
    println!("There are total {} blocks" , blockchain_vec.len());
}