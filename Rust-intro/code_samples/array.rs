// Task : To explain array in rust
// Author : Vigneshwer
// Version : 1.0
// Date : 3 Dec 2016

fn main() {
	
	// Defining an array 
	let rand_array = [1,2,3];

	println!("random array {:?}",rand_array );
	// indexing starts with 0
	println!("random array 1st element {}",rand_array[0] );
	println!("random array length {}",rand_array.len() );
	// last two elements
	println!("random array {:?}",&rand_array[1..3] );
}