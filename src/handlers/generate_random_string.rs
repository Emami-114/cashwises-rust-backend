
use rand::Rng;

pub fn generate_random_string() -> String {
    let mut rng = rand::thread_rng();
    let random_string: String = rng.gen_range(1111..9999).to_string();
    // .sample_iter(&Number)
    // .take(length)
    // // .map(char::from)
    // .collect();
    random_string
}
