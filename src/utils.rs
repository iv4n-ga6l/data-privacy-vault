use rand::{distributions::Alphanumeric, Rng};

pub fn generate_token(data: &str) -> String {
    let random_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();

    format!("{}-{}", data.len(), random_string)
}
