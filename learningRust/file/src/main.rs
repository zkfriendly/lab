mod file_data_loader;
mod random_data_loader;
mod sort;

use sort::merge_sort;

fn main() {
    let mut data = file_data_loader::load_1("test_data.txt");
    let mut data = file_data_loader::load_2("test_data.txt");

    let mut data = match file_data_loader::load_3("test_data.txt") {
        Ok(d) => d,
        Err(er) => panic!("Could not load file, {er}"),
    };

    println!("Is sorted before: {}", merge_sort::is_sorted(&data));

    data = merge_sort::sort(&mut data);

    println!("Is sorted after: {}", merge_sort::is_sorted(&data));
}
