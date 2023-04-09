fn binary_search(array: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = array.len() - 1;

    while low <= high {
        let mid = (low + high) / 2;

        if array[mid] == target {
            return Some(mid);
        } else if array[mid] < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    None
}

fn main() {
    let array = [1, 3, 5, 7, 9];
    let target = 5;
    let index = binary_search(&array, target);

    match index {
        Some(i) => println!("Found {} at index {}", target, i),
        None => println!("{} not found", target),
    }
}
