use rand::distributions::{Alphanumeric, DistString};

pub fn generate_auth_token(length: usize) -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), length) 
}
