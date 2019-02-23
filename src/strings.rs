pub fn run(){
	let mut hello = String::from("Hey There! Welcome Msg");

	// Get Length
	println!("\nLength : \t{}", hello.len());

	// Append to string (Single Char)
	hello.push('C');

	// Append to string (strings)
	hello.push_str("ount");

	println!("\n{}", hello);

	//Check if string is empty
	println!("\nIs Empty: \t{}", hello.is_empty());

	//Check if substring exists
	println!("\nContains 'Msg' \t{}", hello.contains("Msg"));

	//Replace 
	println!("\nReplace \t{}", hello.replace("Welcome", "BienVenue"));

	//Loop through string by whitespace
	for word in hello.split_whitespace(){
		println!("\t{}", word);
	}

	//Create String with Capacitys
	let mut s = String::with_capacity(10);
	s.push('q');
	s.push('w');

	println!("\n{}", s);

	//Assertion Testing
	assert_eq!(2, s.len());
	assert_eq!(10, s.capacity());
}