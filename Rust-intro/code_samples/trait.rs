// Task : To explain trait in rust
// Author : Vigneshwer
// Version : 1.0
// Date : 3 Dec 2016

use std::{f64};

fn main() {

	// variable of circle datatype
	let mut circle1 = Circle {
		r : 10.0
	};
	println!("Area of circle {}", circle1.area() );
	
	// variable of rectangle datatype
	let mut rect = Rectangle {
		h:10.0,b : 10.0
	};
	println!("Area of rectangle {}", rect.area() );
}

// userdefined datatype rectangle
struct Rectangle {
	h: f64,
	b: f64,
}

// userdefined datatype circle
struct Circle {
	r: f64,
}

// create a functionality for the datatypes 
trait  HasArea {
	fn area(&self) -> f64;
}

// implement area for circle
impl HasArea for Circle {
	fn area(&self) -> f64 {
		3.14 * (self.r *self.r)
	}
}

// implement area for rectangle
impl HasArea for Rectangle {
	fn area(&self) -> f64 {
		self.h *self.b
	}
}