use rand::prelude::*;

fn generate_sample_vector(size: i32, max: i32) -> Vec<i32> {
    let mut vector = vec![];
    let mut range = rand::thread_rng();

    for _ in 0..size {
        vector.push(range.gen_range(0..max));
    }

    vector
}

fn binary_search(vector: &Vec<i32>, search_element: i32) -> Option<i32> {
    let length = vector.len();

    let mut m;
    let (mut l, mut r) = (0, length - 1);

    while l <= r {
        m = l + (r - l) / 2;
        if search_element == vector[m] {
            return Some(m as i32);
        } else if search_element > vector[m] {
            l = m + 1
        } else {
            r = m - 1
        }
    }

    None
}

fn main() {
    let mut sample_vector = generate_sample_vector(10, 20);
    sample_vector.sort();

    println!("Input: {:?}\n", sample_vector);

    let element_index = binary_search(&sample_vector, 12);
    match element_index {
        Some(index) => println!("Element was found on index: {}", index),
        None => println!("-> Element is not present in the vector.")
    }
}