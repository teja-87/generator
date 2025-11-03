use solana_sdk::signature::{Keypair, Signer};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Instant;
use rand::rngs::OsRng;
use num_cpus;

fn main() {
    let cores = num_cpus::get();
    println!("Using {cores} threads...");

    let stop_flag = Arc::new(AtomicBool::new(false));
    let mut handles = Vec::new();
    let time_start = Instant::now();

    for i in 0..cores {
        let stop_flag_clone = Arc::clone(&stop_flag);
        handles.push(thread::spawn(move || {
            println!("Thread {i} started");
            gene(&stop_flag_clone, "flys");
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("all threads stopped");
    println!("time taken: {}", time_start.elapsed().as_secs_f64());
}

fn fast_key_pair() -> Keypair {
    Keypair::generate(&mut rand::rngs::OsRng)
}

fn gene(stop_flag: &Arc<AtomicBool>, name: &str) {
    let mut counter = 0;

    while !stop_flag.load(Ordering::Relaxed) {
        counter += 1;
        let keypair = fast_key_pair();
        let public = keypair.pubkey();
        let pub_str = public.to_string();

        if pub_str.ends_with(name) {
            stop_flag.store(true, Ordering::Relaxed);
            println!("\nFOUND! Public: {}", public);
            println!("Secret: {:?}", keypair.to_bytes());
            println!("Attempts: {}", counter);

            std::fs::write(
                "found.json",
                format!("{{\"pub\": \"{}\", \"secret\": {:?}}}", public, keypair.to_bytes())
            ).unwrap();

            return;
        }
    }
}
