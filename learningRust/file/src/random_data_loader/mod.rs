use rand::Rng;

pub fn load() -> Vec<u32> {
    let count = rand::thread_rng().gen_range(100000..=1000000);
    let mut random_data: Vec<u32> = Vec::new();

    for _ in 0..count {
        random_data.push(rand::thread_rng().gen_range(0..4294967295))
    }

    random_data
}
