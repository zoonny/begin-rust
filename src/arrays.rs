// Arrays - Fixed list where elements are the same data types

use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1,2,3,4,5];

    // Re-assign value
    numbers[2] = 20;

    println!("{:?}", numbers);

    // Get single value
    println!("{}", numbers[0]);

    // Get array length
    println!("Array Length: {}", numbers.len());

    // Arrays are stack allocated
    // size_ov_val: 4 bytes * 5 = 20 bytes
    // println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    // let slice: &[i32] = &numbers;
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);
}