
use ed25519_dalek::ed25519::signature::{ SignerMut};
use ed25519_dalek::{Keypair, PublicKey, Signature};

#[derive(Debug)]
pub struct Transaction{
    pub sender:String,
    pub reciever:String,
    pub amount : i32,
    pub timestamp:i64,
    pub signature : Signature
}

impl Transaction{
    pub fn sign_transaction(sender:String , reciever : String , amount:i32 , timestamp:i64 , keypair:&mut Keypair) -> Transaction{
        let tx = format!("{}{}{}{}" , sender , reciever , amount , timestamp);
        let tx_bytes = tx.as_bytes();
        Transaction { sender: sender, reciever: reciever, amount: amount, timestamp: timestamp, signature: keypair.sign(&tx_bytes) }
    }
    pub fn verify_transaction(&self , public_key:&PublicKey) -> bool{
        let tx_bytes = format!("{}{}{}{}" , self.sender , self.reciever , self.amount , self.timestamp);
        let verification = public_key.verify_strict(tx_bytes.as_bytes(), &self.signature).is_ok();
        verification
    }
}