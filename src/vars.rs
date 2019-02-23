pub fn run(){
	let name = "TFQ";
	let mut age = 5;
	println!("I have {} yrs", age);
	
	age = 40;
	println!("I am {} & I am {} yrs old", name, age);

	// Define Constant
	const ID : i32 = 001;
	println!("ID: ${}", ID);

	// Assign Multiple Vars
	let (my_name, my_age) = ("TFQ", 40);
	println!("{} is {} yrs old", my_name, my_age);
	
}