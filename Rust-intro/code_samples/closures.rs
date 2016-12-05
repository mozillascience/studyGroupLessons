// Task : To explain closures in rust
// Author : Vigneshwer
// Version : 1.0
// Date : 3 Dec 2016

use std::{i32};

fn main() {

	// define a closure
	let sum_num = |x:i32 , y:i32| x+y;
	println!("7 + 8 ={}", sum_num(7,8));

	// example 2
	let num_ten = 10;
	let add_ten = |x:i32| x+num_ten;
	println!("3 + 10 ={}", add_ten(3));
}
