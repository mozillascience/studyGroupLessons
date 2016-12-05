// Task : To explain functions in rust
// Author : Vigneshwer
// Version : 1.0
// Date : 3 Dec 2016

use std::{i32};

fn main() {
	//Calling function 1
	say_hello("Mozilla Science");
	//Calling function 2
	println!("5 + 4 ={}", get_sum(5,4));
}

// function does not return 
fn say_hello(name : &str){
	println!("Hello {}", name);
}

// adds two number and returns an integer
fn get_sum(num1:i32,num2:i32) -> i32 {
	// if semi-colon is not put then that returns 
	num1+num2
}

