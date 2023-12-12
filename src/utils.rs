pub fn get_random_bool(probability: f64) -> bool {
    let mut rng = rand::thread_rng();
    let random_number: f64 = rng.gen();
    return random_number < probability;
}