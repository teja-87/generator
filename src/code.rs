use solana_sdk::pubkey;
use solana_sdk::pubkey::pubkey;

#[tokio::main]

async fn main(){

    let keypair = Keypair::new();
    println!("publickey:{}", keypair.pubkey());
    println!("secretkey:{:?}", keypair.to_bytes());
    println!(keypair)
}