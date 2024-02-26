fn power_set(set: &[i32]) -> Vec<Vec<i32>> {
    if set.is_empty() {
        return vec![vec![]];
    }

    let mut subsets = power_set(&set[1..]);
    let first = set[0];
    let mut new_subsets = subsets.clone();

    for subset in &mut new_subsets {
        subset.push(first);
    }

    subsets.append(&mut new_subsets);
    subsets
}

fn main() {
    let set = vec![5, 2, 3];
    let power_set = power_set(&set);

    println!("Power set: {:?}", power_set);
}
