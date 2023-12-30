use std::fs::File;
use std::io::{self, ErrorKind, Read};

pub fn load_1(path: &str) -> Vec<u32> {
    let file_handle_result = File::open(path);

    let mut file_handle = match file_handle_result {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create(path) {
                Ok(file) => file,
                Err(er) => panic!("Could not create file {er}"),
            },
            other_error => {
                panic!("could not open file {other_error}")
            }
        },
    };

    return parse_raw_data(&mut file_handle);
}

pub fn load_2(path: &str) -> Vec<u32> {
    let mut file_handle = File::open(path).unwrap_or_else(|error| match error.kind() {
        ErrorKind::NotFound => File::create(path).expect("Could not create file"),
        _ => panic!("Could not read file"),
    });

    return parse_raw_data(&mut file_handle);
}

pub fn load_3(path: &str) -> Result<Vec<u32>, io::Error> {
    Ok(parse_raw_data(&mut File::open(path)?))
}

fn parse_raw_data(file_handle: &mut File) -> Vec<u32> {
    let mut raw_data = String::new();

    let raw_data = match file_handle.read_to_string(&mut raw_data) {
        Ok(_) => raw_data,
        Err(er) => panic!("Error reading into memory {er}"),
    };

    let mut nums: Vec<u32> = Vec::new();

    for num_str in raw_data.trim().split_whitespace() {
        nums.push(num_str.parse().expect("Invalid number"));
    }

    nums
}
