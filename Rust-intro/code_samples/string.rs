// Task : To explain string in rust
// Author : Vigneshwer
// Version : 1.0
// Date : 3 Dec 2016

fn main() {
	// declaring a random string
	let rand_string = "I love Mozilla Science <3";

	// printing the length of the string
	println!("length of the string is {}",rand_string.len() );

	// Splits in string
	let (first,second) = rand_string.split_at(7);
	println!("First part : {0} Second part : {1}", first,second );

	// Count using iterator count
	let count = rand_string.chars().count();
	print!("count {}",count );

	println!("__________________________");
	// printing all chars
	let mut chars = rand_string.chars();
	let mut indiv_chars = chars.next();

	loop {
		// Its like switch in c++
		match indiv_chars {
			Some(x) => println!("{}",x ),
			None => break
		}
		indiv_chars = chars.next();
	}

	println!("__________________________");
	// iterate over whitespaces
	let mut iter = rand_string.split_whitespace();
	let mut indiv_word = iter.next();

	loop {
		// Its like switch in c++
		match indiv_word {
			Some(x) => println!("{}",x ),
			None => break
		}
		indiv_word = iter.next();
	}

	println!("__________________________");
	// iterate over next line
	let rand_string2 = "I love \n everything about \n Mozilla <3";
	let mut iter_line = rand_string2.lines();
	let mut indiv_sent = iter_line.next();

	loop {
		// Its like switch in c++
		match indiv_sent {
			Some(x) => println!("{}",x ),
			None => break
		}
		indiv_sent = iter_line.next();
	}

}