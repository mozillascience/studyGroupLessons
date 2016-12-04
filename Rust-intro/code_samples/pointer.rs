// Task : To explain pointers in rust
// Author : Vigneshwer
// Version : 1.0
// Date : 3 Dec 2016

use std::{i32};

fn main() {

	let vect1 = vec![1,2,3];
	
	// Error in case you are doing this in case of non primitive value
	// let vec2 = vec1
	// println!("vec1[0] : {:?}", vec1[0]);

	let prim_val = 1;
	let prim_val2 = prim_val;
	println!("primitive value :- {}", prim_val);

	// passing the ownership to the function
	println!("Sum of vects : {}", sum_vects(&vect1));
	// Able to pass the non primitive data type
	println!("vector 1 {:?}", vect1);
}

// Added a reference in the argument
fn sum_vects (v1: &Vec<i32>) -> i32 {
	// apply a closure and iterator
	let sum = v1.iter().fold(0, |mut sum, &x | {sum += x; sum});
	return sum;
}