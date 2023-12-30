pub fn sort(data: &Vec<u32>) -> Vec<u32> {
    let len = data.len();

    if len <= 1 {
        return data.to_vec();
    }

    let mid = len / 2;

    let left = sort(&data[..mid].to_vec());
    let right = sort(&data[mid..].to_vec());

    let mut i = 0;
    let mut j = 0;

    let mut sorted: Vec<u32> = Vec::new();

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            sorted.push(left[i]);
            i += 1;
        } else {
            sorted.push(right[j]);
            j += 1;
        }
    }

    if i < left.len() {
        while i < left.len() {
            sorted.push(left[i]);
            i += 1
        }
    }

    if j < right.len() {
        while j < right.len() {
            sorted.push(right[j]);
            j += 1
        }
    }

    sorted
}

pub fn is_sorted(data: &Vec<u32>) -> bool {
    match data.len() {
        0 => true,
        len => {
            for i in 0..len - 1 {
                if data[i] > data[i + 1] {
                    return false;
                }
            }
            return true;
        }
    }
}
