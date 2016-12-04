// Task : To explain enum in rust
// Author : Vigneshwer
// Version : 1.0
// Date : 3 Dec 2016

fn main() {
	
	let hulk = Hero::Strong(100);
	let fasty = Hero::Fast;
	//converting from 
	let spiderman = Hero::Info {name:"spiderman".to_owned(),secret:"peter parker".to_owned()};

	get_info(spiderman);
	get_info(hulk);
	get_info(fasty);
}

// declaring the enum 
enum Hero {
	Fast,
	Strong(i32),
	Info {name : String, secret : String}
}

// function to perform for each types
fn get_info(h:Hero){
	match h {
		Hero::Fast => println!("Fast"),
		Hero::Strong(i) => println!("Lifts {} tons",i ),
		Hero::Info {name,secret} => { println!(" name is : {0} secret is  : {1}", name,secret);} ,
	}
}