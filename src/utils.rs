use rand::{distributions::Alphanumeric, prelude::*};

#[allow(unused)]
pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[allow(unused)]
pub fn gen_rand_string(digits: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(digits)
        .map(char::from)
        .collect()
}
