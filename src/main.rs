use rand::Rng;
use std::{thread, time};

fn main() {
    let mut rng = rand::thread_rng();
    let mut args = std::env::args();

    let rand_num: usize = rng.gen_range(1..args.len());

    let half_a_sec = time::Duration::from_millis(500);
    thread::sleep(half_a_sec);
    println!("Shuffling...");
    thread::sleep(half_a_sec);
    println!("🦀");
    thread::sleep(half_a_sec);
    println!("⚙️");
    thread::sleep(half_a_sec);
    println!("🦀");
    thread::sleep(half_a_sec);
    println!("⚙️");
    thread::sleep(half_a_sec);

    println!(
        "Winner: {:?}",
        args.nth(rand_num).expect("Couldn't get chosen element.")
    );
}
