use rand::Rng;

pub fn new() -> i32 {
    let mut rng = rand::thread_rng();
    eprintln!("Generating a secret number.");
    let random_number: i32 = rng.gen_range(0..100);
    random_number
}
