// Task : To explain looping in rust
// Author : Vigneshwer
// Version : 1.0
// Date : 3 Dec 2016

fn main() {
	
	// mutuable variable whose value can be changed
	let mut x =1;
    println!(" Loop even numbers ");
    // Continously loops 
	loop {
		// Check if x is an even number or not
		if (x % 2 == 0){
			println!("{}",x);
			x += 1;
			// goes to the loop again
			continue;
		}
		// exit if the number is greater than 10
		if (x > 10)	{
			break;
		}
		// increment the number when not even
		x+=1;
	}

	let mut y = 1;
	// while loop
    println!("while 1 to 9  ");
	while y < 10 {
		println!("{}",y );
		y +=1;
	}

	let mut z = 1;
	//for loop 
	println!(" For 1 to 9");
	for z in 1 .. 10 {
		println!("{}",z );
	}

}