// Task : To explain tuples in rust
// Author : Vigneshwer
// Version : 1.0
// Date : 3 Dec 2016

use std::{i8};

fn main() {
	
	// Declaring a tuple
	let rand_tuple = ("Mozilla Science Lab", 2016);
	let rand_tuple2 : (&str, i8) = ("Viki",4);

	// tuple operations
	println!(" Name : {}", rand_tuple2.0);
	println!(" Lucky no : {}", rand_tuple2.1);

}