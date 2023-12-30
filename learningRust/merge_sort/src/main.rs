mod random_data_loader;
mod sort;

use sort::merge_sort;

fn main() {
    let mut data = random_data_loader::load();

    println!("Is sorted before: {}", merge_sort::is_sorted(&data));

    data = merge_sort::sort(&mut data);

    println!("Is sorted after: {}", merge_sort::is_sorted(&data));
}
