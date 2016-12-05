// Task : To explain vector in rust
// Author : Vigneshwer
// Version : 1.0
// Date : 3 Dec 2016

fn main() {
	// declaring a vector
	let mut vec1 = vec![1,2,3,4,5];

	// printing element 3 in vector
	println!("Item 3 : {}", vec1[2]);

	// iterating in a vector
	for i in &vec1 {
		println!("{}",i );
	}

	// push an element to vector
	vec1.push(6);
	println!("vector after push {:?}", vec1 );
	// pop an element from vector
	vec1.pop();
	println!("vector after pop {:?}", vec1 );
}