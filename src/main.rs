use solana_sdk::signer::{Signer, keypair::{self, Keypair}};
use std::any::type_name;
use std::thread;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

 fn main() {
    
    let cores = num_cpus::get();
    println!("Using {cores} threads...");

    let stop_flag=Arc::new(AtomicBool::new(false));

    let mut handles=Vec::new();


    for i in 0..cores{


        let stop_flag_clone=Arc::clone(&stop_flag);
        
        handles.push( thread::spawn(move||{
            
            println!("Thread {i} started");

            gene(stop_flag_clone);

    }));
}


        for h in handles{
            h.join().unwrap();
        }
        println!("all threads stopped");
    
   

   
 
}

fn gene(stop_flag: Arc<AtomicBool>){
       let name="rays";
    let mut counter=0;

        while !stop_flag.load(Ordering::Relaxed){
        counter+=1;
        let keypair = Keypair::new();
        let public= keypair.pubkey();
        let stri_key=public.to_string();
        
        if stri_key.ends_with(name)
        {
        stop_flag.store(true, Ordering ::Relaxed);
        println!("Public Key: {}", public);
        println!("Secret Key: {:?}", keypair.to_bytes());
       
        println!("counts: {}", counter);
        break;
        }

      
        }
}