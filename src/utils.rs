use rand::{distributions::Alphanumeric, prelude::*};

pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub async fn gen_rand_string(digits: usize) -> String {
    let mut name = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect::<String>();
    


    name
}
